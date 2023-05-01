fn main() {
    let mut table = HashTable::new();
    table.insert("key1", "value1");
    table.insert("key2", "value2");
    table.insert("key3", "value3");
    table.insert("key4", "value4");
    
    println!("{:?}", table);
    
    let value = table.get("key3");
    println!("Value of key3: {:?}", value);
}

#[derive(Debug)] // HashTable yapısına Debug trait'ini implemente ediyoruz
struct HashTable {
    data: Vec<Option<(String, String)>>,
    size: usize,
}

impl HashTable {
    fn new() -> Self {
        Self {
            data: vec![None; 16], // 16 boyutunda bir hash tablosu oluşturuyoruz
            size: 0,
        }
    }

    fn hash_function(&self, key: &str) -> usize {
        let bytes = key.as_bytes();
        let mut hash = 0;
        for byte in bytes {
            hash = (hash << 5) + hash + (*byte as i32); // Klasik bir hash fonksiyonu
            hash = hash & hash.abs();
        }
        (hash as usize) % self.data.len() // Tablo boyutu ile mod işlemi yaparak indeks değeri elde edilir
    }

    fn insert(&mut self, key: &str, value: &str) {
        let index = self.hash_function(key);
        self.data[index] = Some((key.to_string(), value.to_string()));
        self.size += 1;
    }

    fn get(&self, key: &str) -> Option<&str> {
        let index = self.hash_function(key);
        match &self.data[index] {
            Some((k, v)) => {
                if k == key {
                    Some(v.as_str())
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
