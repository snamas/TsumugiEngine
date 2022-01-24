use std::collections::HashMap;
use std::iter::FilterMap;
use std::vec::IntoIter;

pub struct TsumuHashMap<T>{
    //<usize:入ってくる数値,usize:vector配列のインデックス>
    hashmap:HashMap<usize,usize>,
    pub vector:Vec<Option<T>>,
    //freeは空きが出来た場所を示す。削除が多い場合に有利になるかも
    free:Vec<usize>
}

impl<T> TsumuHashMap<T> {
    pub fn new()->Self{
        TsumuHashMap{
            hashmap: Default::default(),
            vector: vec![],
            free:  vec![]
        }
    }
    pub fn overwrite_insert(&mut self,index:usize,element:T){
        match self.hashmap.get(&index){
            None => {
                match self.free.pop(){
                    None => {
                        self.vector.push(Some(element));
                        let i = self.vector.len()-1;
                        self.hashmap.insert(index,i);
                    }
                    Some(free_index) => {
                        self.vector[free_index] = Some(element);
                        self.hashmap.insert(index,free_index);
                    }
                }
            }
            Some(i) => {
                self.vector[*i] = Some(element);
            }
        }
    }
    pub fn delete(&mut self,index:usize){
        match self.hashmap.remove(&index) {
            None => {
                //todo:Noneに入ったと言うことは元々存在していない物を削除しようとしたということ
            }
            Some(hash_index) => {
                self.vector[hash_index] = None;
                self.free.push(hash_index);
            }
        }
    }
}

impl<T> IntoIterator for TsumuHashMap<T>{
    type Item = T;
    type IntoIter = FilterMap<IntoIter<Option<T>>, fn(Option<T>) -> Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.into_iter().filter_map(|e|{e})
    }
}

impl<T> From<TsumuHashMap<T>> for Vec<T> {
    fn from(val: TsumuHashMap<T>) -> Self {
        val.into_iter().collect::<Vec<T>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::TsumuHashMap;

    #[test]
    fn hashmap_set(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["tsumugi"]);
    }
    #[test]
    fn hashmap_set_multi(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["tsumugi","ringo"]);
    }
    #[test]
    fn hashmap_insert_overwrite(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["tsumu","ringo"]);
    }
    #[test]
    fn hashmap_set_bignum(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(10000000000001,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["tsumugi","ringo","tsumu"]);
    }
    #[test]
    fn hashmap_insert_bignum(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(10000000000001,"tsumugi");
        hashvec.overwrite_insert(1,"ringo");
        hashvec.overwrite_insert(0,"tsumu");
        hashvec.overwrite_insert(10000000000001,"tokyo");
        hashvec.overwrite_insert(1,"kyoto");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["tokyo","kyoto","tsumu"]);
    }
    #[test]
    fn hashmap_delete(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.delete(0);
        assert_eq!(hashvec.vector, vec![None]);
        assert_eq!(hashvec.free, vec![0]);
        hashvec.overwrite_insert(1,"Kyoto");
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["Kyoto"]);
    }
    #[test]
    fn hashmap_delete_duplicate(){
        let mut hashvec = TsumuHashMap::new();
        hashvec.overwrite_insert(0,"tsumugi");
        hashvec.overwrite_insert(1,"Kyoto");
        hashvec.overwrite_insert(2,"Nara");
        hashvec.overwrite_insert(3,"Osaka");
        hashvec.overwrite_insert(100,"Fukuoka");
        hashvec.overwrite_insert(5,"Tokyo");
        hashvec.delete(0);
        hashvec.delete(0);
        hashvec.delete(1);
        hashvec.delete(100);
        hashvec.delete(4);
        hashvec.delete(5);
        assert_eq!(hashvec.vector, vec![None,None,Some("Nara"),Some("Osaka"),None,None]);
        assert_eq!(hashvec.free, vec![0,1,4,5]);
        hashvec.overwrite_insert(1,"Sapporo");
        assert_eq!(hashvec.free, vec![0,1,4]);
        assert_eq!(hashvec.vector, vec![None,None,Some("Nara"),Some("Osaka"),None,Some("Sapporo")]);
        assert_eq!(hashvec.into_iter().collect::<Vec<&str>>(), vec!["Nara","Osaka","Sapporo"]);
    }
}
