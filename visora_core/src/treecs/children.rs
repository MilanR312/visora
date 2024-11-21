use std::{collections::VecDeque, ops::{Index, IndexMut}};



#[derive(Clone, Debug, Hash)]
pub enum Children<T>{
    NoChild,
    SingleChild(T),
    DoubleChild(T,T),
    /// remaining variant, this variant does not mean there are > 2 items in the container
    Other(VecDeque<T>)
}
impl<T> Children<T> {
    pub fn new() -> Self {
        Self::NoChild
    }
    pub fn len(&self) -> usize {
        match self {
            Self::NoChild => 0,
            Self::SingleChild(_) => 1,
            Self::DoubleChild(_, _) => 2,
            Self::Other(x) => x.len()
        }
    }
    pub fn new_from<I: IntoIterator<Item = T>>(data: I) -> Self{
        let mut data = data.into_iter();
        let Some(a) = data.next() else {
            return Self::NoChild;
        };
        let Some(b) = data.next() else {
            return Self::SingleChild(a);
        };
        let array_data: VecDeque<_> = data.collect();
        if array_data.len() == 0 {
            Self::DoubleChild(a, b)
        } else {
            let mut out = VecDeque::with_capacity(2 + array_data.len());
            out.push_back(a);
            out.push_back(b);
            out.extend(array_data.into_iter());
            Self::Other(out)
        }
    }
    
    pub fn get_child_count(&self) -> usize {
        match self {
            Children::NoChild => 0,
            Children::SingleChild(_) => 1,
            Children::DoubleChild(_, _) => 2,
            Children::Other(x) => x.len()
        }
    }
    pub fn get_child(&self, idx: usize) -> Option<&T>{
        match self {
            Children::SingleChild(a) if idx == 0 => Some(a),
            Children::DoubleChild(a, _) if idx == 0 => Some(a),
            Children::DoubleChild(_, a) if idx == 1 => Some(a),
            Children::Other(x) => x.get(idx),
            _ => None,
        }
    }
    pub fn get_child_mut(&mut self, idx: usize) -> Option<&mut T>{
        match self {
            Children::SingleChild(a) if idx == 0 => Some(a),
            Children::DoubleChild(a, _) if idx == 0 => Some(a),
            Children::DoubleChild(_, a) if idx == 1 => Some(a),
            Children::Other(x) => x.get_mut(idx),
            _ => None,
        }
    }
    pub fn get_left(&self) -> Option<&T>{
        match self {
            Children::NoChild => None,
            Children::SingleChild(a) => Some(a),
            Children::DoubleChild(a, _) => Some(a),
            Children::Other(a) => a.get(0)
        }
    }
    pub fn get_left_mut(&mut self) -> Option<&mut T>{
        match self {
            Children::NoChild => None,
            Children::SingleChild(a) => Some(a),
            Children::DoubleChild(a, _) => Some(a),
            Children::Other(a) => a.get_mut(0)
        }
    }
    pub fn get_right(&self) -> Option<&T>{
        match self {
            Children::NoChild => None,
            Children::SingleChild(a) => Some(a),
            Children::DoubleChild(_, a) => Some(a),
            Children::Other(a) => a.back()
        }
    }
    pub fn get_right_mut(&mut self) -> Option<&mut T>{
        match self {
            Children::NoChild => None,
            Children::SingleChild(a) => Some(a),
            Children::DoubleChild(_, a) => Some(a),
            Children::Other(a) => a.back_mut()
        }
    }


    pub fn push_right(&mut self, data: T){
        let old_data = std::mem::replace(self, Children::NoChild);
        let new_data = match old_data {
            Children::NoChild => Children::SingleChild(data),
            Children::SingleChild(a) => Children::DoubleChild(a, data),
            Children::DoubleChild(a, b) => Children::Other([a,b,data].into()),
            Children::Other(mut x) => {
                x.push_back(data);
                Children::Other(x)
            }
        };
        let _ = std::mem::replace(self, new_data);
    }
    pub fn push_left(&mut self, data: T){
        let old_data = std::mem::replace(self, Children::NoChild);
        let new_data = match old_data {
            Children::NoChild => Children::SingleChild(data),
            Children::SingleChild(a) => Children::DoubleChild(data, a),
            Children::DoubleChild(a, b) => Children::Other([data,a,b].into()),
            Children::Other(mut x) => {
                x.insert(0, data);
                Children::Other(x)
            }
        };
        let _ = std::mem::replace(self, new_data);
    }
    pub fn pop_left(&mut self) -> Option<T>{
        let old_data = std::mem::replace(self, Children::NoChild);
        let (new_data, to_return) = match old_data {
            Children::NoChild => (Children::NoChild, None),
            Children::SingleChild(a) => (Children::NoChild, Some(a)),
            Children::DoubleChild(a, b) => (Children::SingleChild(b), Some(a)),
            Children::Other(mut x) => {
                //since we already have a vec here we dont dealloc everything and throw it on the stack again
                // this so later pushes dont need to realloc again
                let to_return = x.pop_front();
                (Children::Other(x), to_return)
            }
        };
        let _ = std::mem::replace(self, new_data);
        to_return
    }
    pub fn pop_right(&mut self) -> Option<T>{
        let old_data = std::mem::replace(self, Children::NoChild);
        let (new_data, to_return) = match old_data {
            Children::NoChild => (Children::NoChild, None),
            Children::SingleChild(a) => (Children::NoChild, Some(a)),
            Children::DoubleChild(a, b) => (Children::SingleChild(a), Some(b)),
            Children::Other(mut x) => {
                //since we already have a vec here we dont dealloc everything and throw it on the stack again
                // this so later pushes dont need to realloc again
                let to_return = x.pop_back();
                (Children::Other(x), to_return)
            }
        };
        let _ = std::mem::replace(self, new_data);
        to_return
    }

    
}

impl<T: Eq> Children<T>{
    /// remove key if it exists
    pub fn remove(&mut self, item: T) -> Option<T>{
        let old_self = std::mem::replace(self, Children::NoChild);
        let (new_self, to_return) = match old_self {
            Self::NoChild => (Self::NoChild, None),
            Self::SingleChild(a) if a == item => (Self::NoChild, Some(a)),
            Self::SingleChild(x) => (Self::SingleChild(x), None),
            Self::DoubleChild(a, b) if a == item => (Self::SingleChild(b), Some(a)),
            Self::DoubleChild(a, b) if b == item => (Self::SingleChild(a), Some(b)),
            Self::DoubleChild(a, b) => (Self::DoubleChild(a, b), None),
            Self::Other(mut x) => {
                if x.contains(&item){
                    x.retain(|x| *x != item);
                    (Self::Other(x), Some(item))
                } else {
                    (Self::Other(x), None)
                }
            }
        };
        *self = new_self;
        to_return
    }
}

pub struct IntoIter<T>{
    data: Children<T>
}
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_left()
    }
}
pub struct Iter<'a, T>{
    data: &'a Children<T>,
    index: usize,
    /// index 1 larger than last element
    back_index: usize
}
impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.back_index {
            return None;
        }
        let item = self.data.get_child(self.index)?;
        self.index += 1;
        Some(item)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.data.len();
        (len, Some(len))
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T>{
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T>{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.back_index {
            return None;
        }
        if self.back_index == 0 {
            return None;
        }
        self.back_index -= 1;
        self.data.get_child(self.back_index)
    }
}
pub struct IterMut<'a, T>{
    data: &'a mut Children<T>,
    index: usize
}
impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: every element is only visited once 
        let result = unsafe {
            let item = self.data.get_child_mut(self.index)? as *mut _;
            &mut *item
        };
        self.index += 1;
        Some(result)
    }
}

impl<T> Children<T>{
    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter {
            data: self,
        }
    }
    pub fn iter(&self) -> Iter<'_, T>{
        Iter {
            data: self,
            index: 0,
            back_index: self.len()
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T>{
        IterMut {
            data: self,
            index: 0
        }
    }
}

impl<T> IntoIterator for Children<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
impl<'a, T> IntoIterator for &'a Children<T>{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Children<T>{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<T> Index<usize> for Children<T>{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_child(index).expect("index out of range")
    }
}
impl<T> IndexMut<usize> for Children<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_child_mut(index).expect("index out of range")
    }
}


// due to the fact that Other can contain <2 elements we need to implement eq and ord manually
impl<T: PartialEq> PartialEq for Children<T>{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            // TODO: should this be false?
            (Children::NoChild, Children::NoChild) => true,
            (Children::SingleChild(a), Children::SingleChild(b)) => a==b,
            (Children::SingleChild(a), Children::Other(x)) if x.len() == 1 => *a == x[0],
            (Children::DoubleChild(a, b), Children::DoubleChild(c, d)) => *a==*c && *b==*d,
            (Children::DoubleChild(a, b), Children::Other(x)) if x.len() == 2 => *a == x[0] && *b == x[1],
            (Children::Other(x), Children::Other(y)) => x==y,
            _ => false
        }
    }
}
impl<T: Eq> Eq for Children<T>{
}
impl<T: PartialOrd> PartialOrd for Children<T>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut iter1 = self.iter();
        let mut iter2 = other.iter();
        loop {
            match (iter1.next(), iter2.next()){
                (Some(x), Some(y)) => {
                    let cmp = x.partial_cmp(y)?;
                    if let std::cmp::Ordering::Greater | std::cmp::Ordering::Less = cmp {
                        return Some(cmp)
                    }
                },
                (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                (None, None) => return Some(std::cmp::Ordering::Equal)
            }
        }
    }
}
impl<T: Ord> Ord for Children<T>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut iter1 = self.iter();
        let mut iter2 = other.iter();
        loop {
            match (iter1.next(), iter2.next()){
                (Some(x), Some(y)) => {
                    let cmp = x.cmp(y);
                    if let std::cmp::Ordering::Greater | std::cmp::Ordering::Less = cmp {
                        return cmp
                    }
                },
                (Some(_), None) => return std::cmp::Ordering::Greater,
                (None, Some(_)) => return std::cmp::Ordering::Less,
                (None, None) => return std::cmp::Ordering::Equal
            }
        }
    }
}


#[cfg(test)]
mod tests{

    use super::Children;

    #[test]
    fn child_count(){
        let mut data = Children::new();
        assert_eq!(data.get_child_count(), 0);

        data.push_right(1);
        assert_eq!(data.get_child_count(), 1);
        assert!(matches!(data, Children::SingleChild(1)));

        data.push_right(2);
        assert_eq!(data.get_child_count(), 2);
        assert!(matches!(data, Children::DoubleChild(1, 2)));

        data.push_right(3);
        assert_eq!(data.get_child_count(), 3);
        assert!(matches!(data, Children::Other(_)));

        data.push_right(4);
        assert_eq!(data.get_child_count(), 4);
    }

    #[test]
    fn multi_add_children(){
        let data: Children<u8> = Children::new_from([]);
        assert!(matches!(data, Children::NoChild));

        let data = Children::new_from([1]);
        assert_eq!(data.get_child_count(), 1);
        assert!(matches!(data, Children::SingleChild(1)));

        let data = Children::new_from([1, 2]);
        assert_eq!(data.get_child_count(), 2);
        assert!(matches!(data, Children::DoubleChild(1, 2)));

        let data = Children::new_from([1,2,3]);
        assert_eq!(data.get_child_count(), 3);
        assert!(matches!(data, Children::Other(_)));
    }
    #[test]
    fn get_left_child(){
        let data: Children<u8> = Children::new_from([]);
        assert_eq!(data.get_left(), None);

        let data = Children::new_from([1,2]);
        assert_eq!(data.get_left(), Some(&1));

        let data = Children::new_from([1,2,3,4,5,6]);
        assert_eq!(data.get_left(), Some(&1));
    }
    #[test]
    fn get_right_child(){
        let data: Children<u8> = Children::new_from([]);
        assert_eq!(data.get_right(), None);

        let data = Children::new_from([1,2]);
        assert_eq!(data.get_right(), Some(&2));

        let data = Children::new_from([1,2,3,4,5,6]);
        assert_eq!(data.get_right(), Some(&6));
    }

    #[test]
    fn get(){
        let data: Children<u8> = Children::new_from([]);
        assert_eq!(data.get_child(0), None);
        assert_eq!(data.get_child(1), None);
    
        let data = Children::new_from([1]);
        assert_eq!(data.get_child(0), Some(&1));
        assert_eq!(data.get_child(1), None);
    
        let data = Children::new_from([1,2]);
        assert_eq!(data.get_child(0), Some(&1));
        assert_eq!(data.get_child(1), Some(&2));
        assert_eq!(data.get_child(2), None);

        let data = Children::new_from([1,2,3]);
        assert_eq!(data.get_child(0), Some(&1));
        assert_eq!(data.get_child(1), Some(&2));
        assert_eq!(data.get_child(2), Some(&3));
        assert_eq!(data.get_child(3), None);
    }

    #[test]
    fn add_back(){
        let mut data = Children::new();
        assert_eq!(data.get_left(), None);

        data.push_right(5);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&5));
    
        data.push_right(2);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&2));
    }

    #[test]
    fn add_front(){
        let mut data = Children::new();
        assert_eq!(data.get_left(), None);

        data.push_left(5);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&5));
    
        data.push_left(2);
        assert_eq!(data.get_left(), Some(&2));
        assert_eq!(data.get_right(), Some(&5));

    }
    #[test]
    fn pop_left(){
        let mut data = Children::new_from([1,2,3]);
        assert_eq!(data.pop_left(), Some(1));
        assert_eq!(data.get_left(), Some(&2));
        assert_eq!(data.pop_left(), Some(2));
        assert_eq!(data.pop_left(), Some(3));
        assert_eq!(data.pop_left(), None);

        data.push_left(5);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&5));
    
        data.push_right(2);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&2));
    }

    #[test]
    fn pop_right(){
        let mut data = Children::new_from([1,2,3]);
        assert_eq!(data.pop_right(), Some(3));
        assert_eq!(data.get_right(), Some(&2));
        assert_eq!(data.pop_right(), Some(2));
        assert_eq!(data.pop_right(), Some(1));
        assert_eq!(data.pop_right(), None);

        data.push_left(5);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&5));
    
        data.push_right(2);
        assert_eq!(data.get_left(), Some(&5));
        assert_eq!(data.get_right(), Some(&2));
    }

    #[test]
    fn into_iter(){
        let data = Children::new_from([1,2,3]);
        let mut iter = data.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter(){
        let data = Children::new_from([1,2,3]);
        let mut iter = data.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut(){
        let mut data = Children::new_from([1,2,3]);
        let mut iter = data.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn index(){
        let mut data = Children::new_from([1,2,3,4]);
        assert_eq!(data[0], 1);
        assert_eq!(data[3], 4);
        *&mut data[0] = 2;
        assert_eq!(data[0], 2);
    }
    #[test]
    #[should_panic]
    fn index_panic(){
        let data = Children::new_from([1]);
        data[1];
    }
    #[test]
    fn eq(){
        macro_rules! tv {
            ($(($a:expr, $b:expr, $c:literal)),+) => {
                $(
                    assert!((Children::new_from($a) == Children::new_from($b) ) == $c);
                )+
            }
        }
        assert_eq!(Children::<u8>::NoChild, Children::NoChild);
        tv!(
            ([1], [1], true),
            ([1], [2], false),
            ([], [1,2], false),
            ([1], [1,2], false),
            ([1,1], [1,2], false),
            ([0,2], [1,2], false),
            ([1,2], [1,2], true),
            ([1,2,3], [1,2,3], true),
            ([1,2,2], [1,2,3], false)
        );
    }
    #[test]
    fn ord(){
        macro_rules! tv {
            ($(($a:expr, $b:expr, $sign:tt)),+) => {
                $(
                    assert!(Children::new_from($a) $sign Children::new_from($b));
                )+
            }
        }
        tv!(
            ([2], [1], >),
            ([1], [2], <),
            ([], [1,2], <),
            ([1], [1,2], <),
            ([1,1], [1,2], <),
            ([5,2], [1,2], >),
            ([1,2,2], [1,2,3], <)
        );
    }

    #[test]
    fn remove(){
        let mut data = Children::new_from([1,2,3,4,5]);
        data.remove(3);
        let mut iter = data.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
}