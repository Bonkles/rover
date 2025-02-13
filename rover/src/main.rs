use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, Error};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli { 
    gers_id: String,
}
#[tokio::main]
async fn main() {
    let args = Cli::parse(); 
    
    // s3 configuration and access stuff.
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::defaults(BehaviorVersion::latest())
    .region(region_provider)
    .load()
    .await;

    let s3 = Client::new(&config);
    // Open the bucket named 'omf-internal-usw2' and list its contents
    let bucket_name = "omf-internal-usw2";
    let GERS = &args.gers_id;
    
    // Parse the first three letters of the GERS ID
    let gers_prefix = &GERS[..3];
    println!("First three letters of GERS ID: {:?}", gers_prefix);
    
    match s3.list_objects_v2().bucket(bucket_name).prefix(format!("testing/gers-id-index-ordered/p1={}/", gers_prefix)).send().await {
        Ok(output) => {
            if let Some(objects) = output.contents {
                println!("Objects in folder '/testing/gers-id-index-ordered/p1={}' in bucket '{}':", gers_prefix, bucket_name);
                for object in objects {
                    if let Some(key) = object.key {
                        println!("{}", key);
                    }
                }
            } else {
                println!("No objects found in folder '/testing/gers-id-index-ordered/p1={}' in bucket '{}'.", gers_prefix, bucket_name);
            }
        }
        Err(error) => {
            eprintln!("Error listing objects in folder '/testing/gers-id-index-ordered/p1={}' in bucket '{}': {:?}", gers_prefix, bucket_name, error);
        }
    }
    
    println!("GERS ID to check: {:?}", GERS);
}
