use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Message, WaitForReplyBrach};
use super::channel;

#[derive(Serialize, Deserialize)]
pub struct StepTaskContactData {
    pub contact_id: Uuid,
    pub ext_id: String,
    pub step_id: Uuid,
    pub flow_id: Uuid,
    pub step_type: String,
    pub step_assign_at : DateTime<Utc>

}

pub async fn process_step(db: &PgPool, data: &StepTaskContactData) {
	
	// if data.step_type == "send_message" {
		

	// 	let send_message = sqlx::query!(
	// 		r#"select m.*, c.to as "next_id?" from step_send_message m
	// 		left join flow_connection c on c.id = m.id
	// 		where m.id = $1"#, data.step_id)
	// 	.fetch_one(db)
	// 	.await.unwrap();


	// 	channel::send_text(
	// 		data.host.clone(), 
	// 		data.token.clone(), 
	// 		data.contact_id, 
	// 		data.ext_id.clone(), 
	// 		data.step_id , 
	// 		send_message.content, 
	// 		db
	// 	).await.unwrap();

	// 	sqlx::query!(r#"insert into "contact_step"
	// 		(step_id, contact_id)
	// 		 values ($1, $2)  "#,
    //         send_message.next_id, data.contact_id)
    //         .execute(db)
    //         .await.unwrap();

    //     // println!("sending message to {} : {}", data.ext_id, send_message.content.clone());

	// } if data.step_type == "wait_for_reply" {

	// 	let branches = sqlx::query_as!(WaitForReplyBrach, 
	// 		"select * from step_wait_for_reply_branch where parent_id = $1 order by created_at, id desc", data.step_id)
	// 		.fetch_all(db)
	// 		.await.unwrap();

	// 	let messages = sqlx::query_as!(Message,
	// 		r#"select * from message
	// 			where contact_id = $1 and created_by = 'contact' and created_at > $2 and text is not null
	// 			order by created_at desc"# , 
	// 			data.contact_id, data.step_assign_at)
	// 		.fetch_all(db)
	// 		.await.unwrap();

	// 	let text = messages.iter().map(|m| m.text.clone().unwrap_or("".to_string()) )
	// 		.collect::<Vec<String>>()
	// 		.join(" ");

	// 	if messages.len() == 0 {
	// 		return ;
	// 	}

	// 	let branch = branches.iter().find(|branch| {
	// 		println!("{} - {}", text, branch.text );
	// 		text.contains(branch.text.as_str())
	// 	});

	// 	if let Some(branch) = branch {
	// 		let con = sqlx::query!("select * from flow_connection where id = $1", branch.id)
	// 			.fetch_one(db).await;

	// 		if let Ok(con) = con {
	// 			sqlx::query!(r#"insert into "contact_step"
	// 				(step_id, contact_id)
	// 				 values ($1, $2)  "#,
	// 	            con.to , data.contact_id)
	// 	            .execute(db)
	// 	            .await.unwrap();
	// 		} else {
	// 			sqlx::query!(r#"insert into "contact_step"
	// 				(step_id, contact_id)
	// 				 values (null, $1)  "#,
	// 	            data.contact_id)
	// 	        .execute(db)
	// 	        .await.unwrap();
	// 		}
	// 	}	
	// } else {

	// }
}