use serde::{Serialize, Deserialize};

use serde_json::Result;

pub fn run() -> Result<Vec<Profile>>  {
    let data = r#"
    [
      {
        "id": 1,
        "type": "personal",
        "details": {
          "firstName": "Juliano",
          "lastName": "Alves",
          "primaryAddress": 7777777
        }
      },
      {
        "id": 2,
        "type": "business",
        "details": {
          "name": "Juliano Business",
          "companyRole": "OWNER",
          "primaryAddress": 8888888
        }
      }
    ]
    "#;

    let profiles: Vec<Profile> = serde_json::from_str(data)?;

    Ok(profiles)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalDetails {
    first_name: String,
    last_name: String,
    primary_address: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BusinessDetails {
    name: String,
    company_role: String,
    primary_address: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Profile {
    Personal {
        id: i32,
        details: PersonalDetails,
    },
    Business {
        id: i32,
        details: BusinessDetails,
    },
}