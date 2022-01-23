use std::collections::HashMap;

pub struct TsumuHashMap<T>{
    hashmap:HashMap<usize,usize>,
    pub vector:Vec<T>
}

impl<T> TsumuHashMap<T> {
    pub fn new()->Self{
        TsumuHashMap{
            hashmap: Default::default(),
            vector: vec![]
        }
    }
    pub fn overwrite_insert(&mut self,index:usize,element:T){
        match self.hashmap.get(&index){
            None => {
                self.vector.push(element);
                let i = self.vector.len()-1;
                self.hashmap.insert(index,i);
            }
            Some(i) => {
                self.vector[*i] = element;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TsumuHashMap;

    #[test]
    fn hashmap_set(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        assert_eq!(hashvec.vector, vec!["tsumugi"]);
    }
    #[test]
    fn hashmap_set_multi(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        assert_eq!(hashvec.vector, vec!["tsumugi","ringo"]);
    }
    #[test]
    fn hashmap_insert_overwrite(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        assert_eq!(hashvec.vector, vec!["tsumu","ringo"]);
    }
    #[test]
    fn hashmap_set_bignum(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(10000000000001,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        assert_eq!(hashvec.vector, vec!["tsumugi","ringo","tsumu"]);
    }
    #[test]
    fn hashmap_insert_bignum(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(10000000000001,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        hashvec.overwrite_insert(10000000000001,"tokyo");
        hashvec.overwrite_insert(1,"kyoto");
        assert_eq!(hashvec.vector, vec!["tokyo","kyoto","tsumu"]);
    }
}
