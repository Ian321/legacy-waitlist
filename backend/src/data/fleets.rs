use crate::{core::esi::{ESIClient, ESIError, ESIScope, self, fleet_members::ESIFleetMember}, util::madness::Madness};
use eve_data_core::TypeDB;
use serde::{Deserialize, Serialize};
use crate::util::types::System;

#[derive(Debug)]
pub struct Wing {
    pub name: String,
    pub squads: Vec<Squad>
}

#[derive(Debug)]
pub struct Squad {
    pub name: String,
    pub map_to: Option<String>
}

pub async fn load_default_squads()
 -> Vec<Wing> {
    let mut wings = Vec::new();

    let on_grid_squads = vec![
        Squad {
            name: "Logistics".to_string(),
            map_to: Some("logi".to_string())
        },
        Squad {
            name: "Bastion".to_string(),
            map_to: Some("bastion".to_string())
        },
        Squad {
            name: "CQC".to_string(),
            map_to: Some("cqc".to_string())
        },
        Squad {
            name: "Sniper".to_string(),
            map_to: Some("sniper".to_string())
        },
        Squad {
            name: "Starter".to_string(),
            map_to: Some("starter".to_string())
        },
        Squad {
            name: "Alts".to_string(),
            map_to: Some("alt".to_string())
        },
        Squad {
            name: "Box 1".to_string(),
            map_to: None
        },
        Squad {
            name: "Box 2".to_string(),
            map_to: None
        },
        Squad {
            name: "Box 3".to_string(),
            map_to: None
        },
        Squad {
            name: "Box 4".to_string(),
            map_to: None
        }
    ];

    wings.push(Wing {
        name: "On Grid".to_string(),
        squads: on_grid_squads
    });


    let off_grid_squads = vec![
        Squad {
            name: "Scout 1".to_string(),
            map_to: None
        },
        Squad {
            name: "Scout 2".to_string(),
            map_to: None
        },
        Squad {
            name: "Other".to_string(),
            map_to: None
        },
    ];

    wings.push(Wing {
        name: "Off Grid".to_string(),
        squads: off_grid_squads
    });

    wings
}



#[derive(Debug, Deserialize)]
pub struct FleetInfo {
    pub fleet_id: i64,
    pub fleet_boss_id: i64
}

#[derive(Debug, Deserialize)]
struct WingInfo {
    id: i64,
}

pub async fn delete_all_wings(
    esi_client: &ESIClient,
    fleet: &FleetInfo
) -> Result<(), ESIError> {
    let current_wings : Vec<WingInfo> = esi_client.get(
        &format!("/v1/fleets/{}/wings", fleet.fleet_id),
        fleet.fleet_boss_id,
        ESIScope::Fleets_ReadFleet_v1,
    )
    .await?;

    for wing in current_wings {
        esi_client.delete(
            &format!("/v1/fleets/{}/wings/{}", fleet.fleet_id, wing.id),
            fleet.fleet_boss_id,
            ESIScope::Fleets_WriteFleet_v1
        )
        .await?;
    }

    Ok(())
}

pub async fn set_default_motd(
    db: &crate::DB,
    esi_client: &ESIClient,
    fleet: &FleetInfo
) -> Result<(), ESIError> {
    let base_motd_template = std::fs::read_to_string("./data/motd.dat").expect("Could not load motd.dat");

    let mut result = base_motd_template;

    if let Some(fc) = sqlx::query!("SELECT name FROM character WHERE id=$1", fleet.fleet_boss_id)
        .fetch_optional(db)
        .await? {
            result = result.replace("{fc_id}", &format!("1379//{}", fleet.fleet_boss_id));
            result = result.replace("{fc_name}", &format!("{}", fc.name));
        }

        let members: Vec<ESIFleetMember> = esi::fleet_members::get(esi_client, fleet.fleet_id, fleet.fleet_boss_id).await?.into_iter().collect();
        let mut boss_system_id: Option<i64> = None;
        for member in &members {
            if member.character_id == fleet.fleet_boss_id {
                boss_system_id = Some(member.solar_system_id);
                break;
            }
        }

        let boss_system_name = match TypeDB::name_of_system(boss_system_id.unwrap()) {
            Ok(name) => name.to_string(),
            _ => "Unknown".to_string()
        };

        result = result.replace("{fc_system_id}", &format!("{}", boss_system_id.unwrap()));
        result = result.replace("{fc_system_name}", &format!("{}", boss_system_name));


    #[derive(Debug, Serialize)]
    struct UpdateFleetBody{
        is_free_move: bool,
        motd: String
    }

    esi_client.put(
        &format!("/v1/fleets/{}", fleet.fleet_id),
        &UpdateFleetBody {
            is_free_move: false,
            motd: result
        },
        fleet.fleet_boss_id,
        ESIScope::Fleets_WriteFleet_v1
    )
    .await?;

    Ok(())
}
