use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use crate::service::flow::{StepTaskContactData, process_step};

pub fn init (db: PgPool) {
 
	tokio::spawn(async move {
        loop {
            //todo
            // let contacts = sqlx::query_as!(StepTaskContactData,
            //     r#"select co.id as contact_id, co.ext_id, co.channel_id,
            //         cs.step_id as "step_id!", cs.created_at as "step_assign_at!",
            //         s.flow_id, s.t as step_type
            //     from contact co
            //     join (select DISTINCT ON (contact_id) contact_step.*
            //         from contact_step
            //         order by contact_id, created_at desc
            //       ) as cs
            //         on co.id = cs.contact_id
            //     join step s on s.id = cs.step_id
            //     where cs.step_id is not null"#
            // ).fetch_all(&db).await;

            // if let Ok(contacts) = contacts {
            //     for c in contacts.iter().to_owned() {
            //         process_step(&db, c).await;
            //     }
            // }

            sleep(Duration::from_millis(500)).await;
        }
            
    });

}