use anyhow::{anyhow, Result};
use michel_core::MichelPersistence;
use milli::documents::{DocumentsBatchBuilder, DocumentsBatchReader};
use milli::{heed, update, Filter, Search, SearchResult};
use std::collections::HashMap;
use std::io::Cursor;
use tempdir;
use tempdir::TempDir;

pub(crate) type Document = serde_json::Map<String, serde_json::Value>;

const MAX_OS_PAGE_SIZE: usize = 16_777_216;
const MAX_POSSIBLE_SIZE: usize = 2_000_000_000;
const MAX_MAP_SIZE: usize = MAX_POSSIBLE_SIZE - (MAX_POSSIBLE_SIZE % MAX_OS_PAGE_SIZE);

/**
    Big inspiration (with consent from the owner) here : https://github.com/GregoryConrad/mimir/blob/main/packages/mimir/native/src/embedded_milli/v1.rs
*/

pub struct MilliPersistence {
    indexes: HashMap<String, milli::Index>,
}

impl MilliPersistence {
    pub fn new() -> Result<MilliPersistence> {
        Ok(MilliPersistence {
            indexes: HashMap::new(),
        })
    }

    fn get_index(&self, index: michel_core::Index) -> Option<&milli::Index> {
        return self.indexes.get(&index.name);
    }

    fn create_index(&mut self, michel_index: michel_core::Index) -> Result<()> {
        if self.indexes.contains_key(&michel_index.name) {
            return Err(anyhow!("index already exists"));
        }

        let path = TempDir::new(&michel_index.name)?;

        std::fs::create_dir_all(&path)?;
        let mut options = heed::EnvOpenOptions::new();
        options.map_size(MAX_MAP_SIZE);

        let index = milli::Index::new(options, &path).map_err(anyhow::Error::from)?;

        self.indexes.insert(michel_index.name, index);

        Ok(())
    }
}

impl MichelPersistence for MilliPersistence {
    fn add_document(&self, index: michel_core::Index, document: Document) -> Result<()> {
        let milli_index = self.get_index(index).ok_or(anyhow!("index not created"))?;

        // Create a batch builder to convert json_documents into milli's format
        let mut builder = DocumentsBatchBuilder::new(Vec::new());

        builder.append_json_object(&document)?;

        // Flush the contents of the builder and retreive the buffer to make a batch reader
        let buff = builder.into_inner()?;
        let reader = DocumentsBatchReader::from_reader(Cursor::new(buff))?;

        // Create the configs needed for the batch document addition
        let indexer_config = update::IndexerConfig::default();
        let indexing_config = update::IndexDocumentsConfig::default();

        // Make an index write transaction with a batch step to index the new documents
        let mut wtxn = milli_index.write_txn()?;
        let (builder, indexing_result) = update::IndexDocuments::new(
            &mut wtxn,
            milli_index,
            &indexer_config,
            indexing_config,
            |_| (),
            || false,
        )?
        .add_documents(reader)?;
        indexing_result?; // check to make sure there is no UserError
        builder.execute()?;

        wtxn.commit().map_err(Into::into)
    }

    fn search_document(
        &self,
        index: michel_core::Index,
        query: String,
        limit: Option<u32>,
    ) -> Result<Vec<Document>> {
        // Create the search
        let milli_index = self.get_index(index).ok_or(anyhow!("index not found"))?;

        let rtxn = milli_index.read_txn()?;
        let mut search = Search::new(&rtxn, milli_index);

        // Configure the search based on given parameters
        search.query(query);
        search.limit(limit.unwrap_or(u32::MAX).try_into()?);

        // Get the documents based on the search results
        let SearchResult { documents_ids, .. } = search.execute()?;
        let fields_ids_map = milli_index.fields_ids_map(&rtxn)?;
        milli_index
            .documents(&rtxn, documents_ids)?
            .iter()
            .map(|(_id, doc)| milli::all_obkv_to_json(*doc, &fields_ids_map))
            .map(|r| r.map_err(anyhow::Error::from))
            .collect()
    }
}
