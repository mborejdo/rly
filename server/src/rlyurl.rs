use harsh::Harsh;
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::urls;

#[table_name = "urls"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct RlyUrl {
    pub id: Option<i32>,
    pub key: String,
    pub url: String
}

impl RlyUrl {

    pub fn generate_id(connection: &MysqlConnection) -> String {
        let v = urls::table.order(urls::id.desc()).load::<RlyUrl>(connection).unwrap();
        // TODO might be empty if database
        let id = v[0].id.unwrap();
        let next = id + 1;
        println!("{}", next);
        Harsh::default().encode(&[next as u64])
    }

    pub fn create(rlyurl: RlyUrl, connection: &MysqlConnection) -> RlyUrl {
        diesel::insert_into(urls::table)
            .values(&rlyurl)
            .execute(connection)
            .expect("Error creating new url");

            urls::table.order(urls::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<RlyUrl> {
        urls::table.order(urls::id).load::<RlyUrl>(connection).unwrap()
    }

    pub fn update(id: i32, rlyurl: RlyUrl, connection: &MysqlConnection) -> bool {
        diesel::update(urls::table.find(id)).set(&rlyurl).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(urls::table.find(id)).execute(connection).is_ok()
    }

    pub fn find(key: String, connection: &MysqlConnection) -> Result<Self, Box<dyn std::error::Error>> {
        let url = urls::table.filter(urls::key.eq(key)).first(connection)?;
        Ok(url)
    }
}