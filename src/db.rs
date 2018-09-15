use mongodb::Client;

const DEFAULT_MONGO_ADDRESS: &str = "localhost";

pub struct MongoClient {
    address: String,
}

impl MongoClient {
    pub fn connect(&self) -> std::sync::Arc<mongodb::ClientInner> {
        let database_url = match env::var(self.address) {
            Ok(value) => value,
            Err(_) => DEFAULT_MONGO_ADDRESS.to_string(),
        };

        Client::connect(&database_url, 27017).expect("Failed to initialize client.")
    }
}

pub struct Collection {
    client: &Client,
    collection: u8,
}

impl Collection {
    fn new(client: &Client, db: String, collection: String) -> Collection {
        Collection {
            client,
            collection: client.db(db).collection(collection),
        }
    }

    fn find(
        &self,
        doc_id: String,
    ) -> Result<std::option::Option<bson::ordered::OrderedDocument>, io::Error> {
        let id = ObjectId::with_string(&doc_id).unwrap();
        self.collection.find_one(Some(doc! { "_id" => id }), None)
    }
}
