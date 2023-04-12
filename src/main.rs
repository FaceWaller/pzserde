use serde::ser::SerializeStruct;
use serde_json::Value;
use serde::de::{self};
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Serialize, Deserialize, Debug)]
struct Animal {
    age: i64,
    name: String
}

 // 普通转换    
#[test]
fn test_general() {
    let json_str = r#" {        
        "age": 3,   
        "name": "dog"
    }"#;    
    let dog: Animal = serde_json::from_str(json_str).unwrap();  // 反序列化    
    println!("dog {:?}", dog);    
    let str = serde_json::to_string(&dog).unwrap(); // 序列化    
    println!("dog {}", str);  
}

// ==========================================================================================

#[derive(Serialize, Deserialize, Debug)]
struct Personal {    
    #[serde(default)]                       // 指定默认值    
    age: i32,    
    #[serde(default = "get_id")]            // 指定获取默认值的方法    
    id: i32,   
    #[serde(rename = "name")]               // 修改序列化和反序列化的字段名    
    personal_name: String,    
    #[serde(rename(serialize = "s_hobby"))] // 修改序列化字段名    
    hobby: String,    
    #[serde(default,rename(deserialize = "d_address"))] // 修改反序列化名同时允许默认值    
    address: String,    
    #[serde(alias = "d_gender", alias = "dd_gender")]  // 反序列化别名，允许配置多个    
    gender: String,
    #[serde(deserialize_with = "deserialize_with_string")] // 反序列化时会通过自定义函数
    height: i64,
    #[serde(serialize_with = "serialize_field")]    // 序列化时会通过自定义函数
    weight: i64,
}

fn get_id() -> i32 {    
    100
}

// 可以通过该函数兼容不同类型或者直接报错
fn deserialize_with_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{           
    let v: Value = Deserialize::deserialize(deserializer)?;

    if v.is_string() {
        let r = v.as_str().ok_or(de::Error::custom("转换失败"))?;
        let r: i64 = r.parse().map_err(|e| de::Error::custom("转换失败"))?;
        Ok(r)
    } else if v.is_i64() {
        let r = v.as_i64().ok_or(de::Error::custom("转换失败"))?;
        Ok(r)
    } else {
        Ok(0)
    }
}

fn serialize_field<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // 在这里实现自定义的序列化逻辑，将 i64 类型的 value 转换为字符串类型
    serializer.serialize_str(&value.to_string())
}

#[test]
fn test_made() {  
    let json_str= r#" {        
        "name": "nick",        
        "hobby": "baseball",        
        "address": "beijing",
        "d_gender": "d_man" ,
        "height": "177",
        "weight": 130
    }"#;    
    let nick: Personal = serde_json::from_str(json_str).unwrap();    
    println!("nick {:?}",nick);    
    let str = serde_json::to_string(&nick).unwrap();    
    println!("nick {}", str);
}


// ==========================================================================================
#[derive(Debug, Clone)]
struct Cat {
    age: i64,
    name: String,
}

impl<'de> Deserialize<'de> for Cat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 在这里实现自定义的反序列化逻辑
        // 使用 deserializer 提供的方法来读取输入数据并解析为 Cat 结构体

        // 例如，假设输入数据为 JSON 格式，可以使用 serde_json 库进行反序列化
        let cat_json: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
        
        // 提取 age 和 name 字段的值
        let age = cat_json["age"].as_i64().ok_or_else(|| serde::de::Error::custom("Missing age"))?;
        let name = cat_json["name"].as_str().ok_or_else(|| serde::de::Error::custom("Missing name"))?;
        
        // 创建 Cat 结构体并返回
        Ok(Cat {
            age,
            name: name.to_string(),
        })
    }
}

impl Serialize for Cat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 在这里实现自定义的序列化逻辑
        // 使用 serializer 提供的方法来将 Cat 结构体序列化为输出数据

        // 例如，假设输出数据为 JSON 格式，可以使用 SerializeStruct 来创建 JSON 对象
        let mut cat_struct = serializer.serialize_struct("Cat", 2)?;
        cat_struct.serialize_field("age", &self.age)?;
        cat_struct.serialize_field("name", &self.name)?;
        cat_struct.end()
    }
}

#[test]
fn test_custom() {
        // 自定义序列化    
    let json_str = r#" {        
        "age": 100,        
        "name": "Dog"    
    }"#;    
    let cc: Cat = serde_json::from_str(json_str).unwrap();
    println!("cc {:?}", cc);       
    let str = serde_json::to_string(&cc).unwrap();
    println!("cc {:?}", str);
}

// ==========================================================================================


fn main() {    
    
}

