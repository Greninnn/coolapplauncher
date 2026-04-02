use steamgriddb_api::{images::Image, query_parameters::Platform, Client, QueryType};

pub async fn get_image_for_id(id: &str) -> Vec<Image> {
    let client = Client::new("");
    client
        .get_images_for_platform_id(&Platform::Steam, id, &QueryType::Grid(None))
        .await
        .unwrap()
}
