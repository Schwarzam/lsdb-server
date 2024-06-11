mod paths;

#[cfg(test)]
mod parser {
    use lsdb_server::loaders::parquet;
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn process_and_return_parquet_file() {
        let file_path = paths::get_testdata_path("Npix=754.parquet");
        let mut params = HashMap::new();

        params.insert("columns".to_string(), "RA,DEC".to_string());
        params.insert("filters".to_string(), "RA>=30.1241,DEC<=-30.3".to_string());

        let result = parquet::parquet::process_and_return_parquet_file(
            file_path.to_str().unwrap(), 
            &params
        ).await;

        // Add assertions to verify the result
        assert!(result.is_ok(), "The function should return Ok");
        let _data = result.unwrap();
        // Additional assertions based on expected data
    }
    

}