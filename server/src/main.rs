pub mod types;

use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use radix_engine_interface::prelude::*;
use radix_transactions::builder::*;
use radix_transactions::manifest::decompiler::decompile_with_known_naming;
use scrypto::prelude::*;
use types::NFTData;

#[post("/manifest")]
async fn get_manifest(body: web::Json<NFTData>) -> impl Responder {
    let network = NetworkDefinition::stokenet();
    let manifest = ManifestBuilder::new().create_non_fungible_resource(
        OwnerRole::None,
        NonFungibleIdType::Integer,
        true,
        NonFungibleResourceRoles::default(),
        metadata! {},
        Some(indexmap! {
            NonFungibleLocalId::integer(1) => NFTData {
                name: body.clone().name,
                description:body.clone().description,
                image_uri: body.clone().image_uri,
            },
        }),
    );

    let obj_names = manifest.object_names();
    let manifest_string =
        decompile_with_known_naming(&manifest.build().instructions, &network, obj_names)
            .unwrap_or("".to_string());
    HttpResponse::Ok().body(manifest_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(get_manifest)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
