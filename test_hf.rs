fn main() {
    let json = r#"
    {
     "_id": "69c2690a115252e673ca991f", 
     "id": "CohereLabs/cohere-transcribe-03-2026", 
     "likes": 792, 
     "trendingScore": 419, 
     "private": false, 
     "downloads": 96615, 
     "createdAt": "2026-03-24T10:35:54.000Z", 
     "modelId": "CohereLabs/cohere-transcribe-03-2026" 
   }
    "#;
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    println!("downloads: {:?}", v.get("downloads").and_then(|v| v.as_u64()));
    println!("likes: {:?}", v.get("likes").and_then(|v| v.as_u64()));
    println!("createdAt: {:?}", v.get("createdAt").and_then(|v| v.as_str()));
    println!("trendingScore: {:?}", v.get("trendingScore").and_then(|v| v.as_u64()));
}
