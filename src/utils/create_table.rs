use sqlx::{Pool, Postgres};

pub async fn create_table(pool: &Pool<Postgres>) -> Result<String, Box<dyn std::error::Error>> {
    let check_table: &str = "SELECT EXISTS (
    SELECT 1
    FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'client_calibri'
);";

    let row: (bool,) = sqlx::query_as(&check_table).fetch_one(pool).await?;
    let table_exists: bool = row.0;

    if table_exists {
        return Ok("Table already exists".to_string());
    }

    if !table_exists {
        let client_calibri: &str = "CREATE TABLE public.client_calibri (
            id serial4 NOT NULL,
            site_id int8 NOT NULL,
            sitename varchar(255) NULL,
            domains varchar(255) NULL,
            active varchar(255) NOT NULL,
            license_start text NULL,
            license_end text NULL,
            not_enough_money bool NULL,
            CONSTRAINT client_calibri_id_key UNIQUE (id),
            CONSTRAINT client_calibri_pkey PRIMARY KEY (id, site_id),
            CONSTRAINT client_calibri_site_id_key UNIQUE (site_id));";

        sqlx::query(&client_calibri)
            .execute(pool)
            .await
            .expect("Error creating table");

        let call: &str = "CREATE TABLE public.calls (
            id serial4 NOT NULL,
            client_calibri_site_id_fk int8 NOT NULL,
            call_id int8 NOT NULL,
            date varchar(255) NOT NULL,
            channel_id int8 NULL,
            is_lid bool NULL,
            name_type text NULL,
            traffic_type text NULL,
            landing_page text NULL,
            conversations_number int8 NULL,
            call_status varchar(255) NULL,
            source text NULL,
            CONSTRAINT calls_pkey PRIMARY KEY (id),
            CONSTRAINT unique_call_id UNIQUE (call_id),
            CONSTRAINT calls_client_calibri_site_id_fk_fkey FOREIGN KEY (client_calibri_site_id_fk) REFERENCES public.client_calibri(site_id));";

        sqlx::query(&call)
            .execute(pool)
            .await
            .expect("Error creating table");

        let email: &str = "CREATE TABLE public.email (
            id serial4 NOT NULL,
            client_calibri_site_id_fk int8 NOT NULL,
            email_id int8 NOT NULL,
            date varchar(255) NULL,
            source text NULL,
            is_lid bool NULL,
            traffic_type text NULL,
            landing_page text NULL,
            lid_landing text NULL,
            conversations_number int8 NULL,
            CONSTRAINT email_pkey PRIMARY KEY (id),
            CONSTRAINT unique_email_id UNIQUE (email_id),
            CONSTRAINT email_client_calibri_site_id_fk_fkey FOREIGN KEY (client_calibri_site_id_fk) REFERENCES public.client_calibri(site_id) );";

        sqlx::query(&email)
            .execute(pool)
            .await
            .expect("Error creating table");

        let phone: &str = "CREATE TABLE public.phone (
                id serial4 NOT NULL,
                client_calibri_site_id_fk int8 NOT NULL,
                number _text NULL,
                CONSTRAINT phone_pkey PRIMARY KEY (id),
                CONSTRAINT unique_client_calibri_site_id_fk UNIQUE (client_calibri_site_id_fk),
                CONSTRAINT phone_client_calibri_site_id_fk_fkey FOREIGN KEY (client_calibri_site_id_fk) REFERENCES public.client_calibri(site_id));";

        sqlx::query(&phone)
            .execute(pool)
            .await
            .expect("Error creating table");
    }

    Ok("Table created successfully!".to_string())
}
