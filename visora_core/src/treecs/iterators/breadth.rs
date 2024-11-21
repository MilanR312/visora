use std::{collections::VecDeque, marker::PhantomData};

use crate::treecs::{query::QueryAble, EntityKey, Treecs};

use super::{QueryIter, WorldIter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BreadthInfo{
    MoveUp,
    Other
}

// div
//  -> p
//  -> p
// div    </div>  add
//        </div>  render
//  <p>   </div></p> add
//<p><p>  </div></p></p> add
//   <p>  </div></p></p> render
//        </div></p></p> render

/**
 * <div>
 *   <div>
 *      <p></p>
 *      <input>
 *   </div>
 *   <p></p>
 * </div>
 * 
 * div, div, p, /p, input, /div, p, /p, /div
 * 
 * 0 0 
 * 
 */
/*
let (info, key) = self.entity_stack.pop_front()?;
        let BreadthInfo::Other = info else {
            return Some((info, key));
        };
        let linkdata = self.world.linkdata(key)?;
        self.entity_stack
            .extend(linkdata.children().iter().copied().map(|x| (BreadthInfo::Other, x)));
        //if linkdata.children().len() != 0 {
            self.entity_stack.push_back((BreadthInfo::MoveUp, key));
        //}
        println!("stack is now {key:?} -> {:?}", self.entity_stack);
        Some((BreadthInfo::Other, key)) */
pub trait Dir{}
pub struct Forward;
impl Dir for Forward{}
pub struct Reversed;
impl Dir for Reversed{}


pub struct BreadthIter<'world, D: Dir = Forward> {
    world: &'world Treecs,
    entity_stack: VecDeque<(BreadthInfo, EntityKey)>,
    dir: PhantomData<D>
}    

impl<'world> Iterator for BreadthIter<'world, Forward> {
    type Item = (BreadthInfo, EntityKey);
    fn next(&mut self) -> Option<Self::Item> {
        let (info, key) = self.entity_stack.pop_back()?;
        match info {
            BreadthInfo::Other => {
                self.entity_stack.push_back((BreadthInfo::MoveUp, key));
                let linkdata = self.world.linkdata(key)?;
                //println!("{key:?} -> children = {:?}", linkdata.children());
                self.entity_stack.extend(linkdata.children().iter().rev().map(|x| (BreadthInfo::Other, *x)));
                Some((BreadthInfo::Other, key))
            },
            BreadthInfo::MoveUp => Some((BreadthInfo::MoveUp, key))
        }
    }
}
impl<'world> Iterator for BreadthIter<'world, Reversed> {
    type Item = (BreadthInfo, EntityKey);
    fn next(&mut self) -> Option<Self::Item> {
        let (info, key) = self.entity_stack.pop_back()?;
        match info {
            BreadthInfo::Other => {
                self.entity_stack.push_back((BreadthInfo::MoveUp, key));
                let linkdata = self.world.linkdata(key)?;
                //println!("{key:?} -> children = {:?}", linkdata.children());
                self.entity_stack.extend(linkdata.children().iter().map(|x| (BreadthInfo::Other, *x)));
                Some((BreadthInfo::Other, key))
            },
            BreadthInfo::MoveUp => Some((BreadthInfo::MoveUp, key))
        }
    }
}
impl<'world, Q: QueryAble> QueryIter<'world, Q> for BreadthIter<'world, Forward>
{
    type Info = (BreadthInfo, EntityKey);
    fn transform(&self, key: EntityKey) -> Option<Q::Output<'world>> {
        Q::get(&self.world, key)
    }
}
impl<'world, Q: QueryAble> QueryIter<'world, Q> for BreadthIter<'world, Reversed>
{
    type Info = (BreadthInfo, EntityKey);
    fn transform(&self, key: EntityKey) -> Option<Q::Output<'world>> {
        Q::get(&self.world, key)
    }
}
impl<'world, Q: QueryAble> WorldIter<'world, Q> for BreadthIter<'world, Forward> {
    fn new(world: &'world Treecs) -> Self {
        let mut entity_stack = VecDeque::new();
        entity_stack.push_back((BreadthInfo::Other, world.root()));
        Self {
            world,
            entity_stack,
            dir: PhantomData
        }
    }
    fn restart(self) -> Self {
        let Self{world, ..} = self;
        <Self as WorldIter<'world, Q>>::new(world)
    }
    
}
impl<'world, Q: QueryAble> WorldIter<'world, Q> for BreadthIter<'world, Reversed> {
    fn new(world: &'world Treecs) -> Self {
        let mut entity_stack = VecDeque::new();
        entity_stack.push_back((BreadthInfo::Other, world.root()));
        Self {
            world,
            entity_stack,
            dir: PhantomData
        }
    }
    fn restart(self) -> Self {
        let Self{world, ..} = self;
        <Self as WorldIter<'world, Q>>::new(world)
    }
}
impl<'world> BreadthIter<'world, Forward>{
    pub fn reverse<Q: QueryAble>(self) -> BreadthIter<'world, Reversed>{
        <BreadthIter<'world, Reversed> as WorldIter<'world, Q>>::new(self.world)
    }
}
impl<'world> BreadthIter<'world, Reversed>{
    pub fn reverse<Q: QueryAble>(self) -> BreadthIter<'world, Forward>{
        <BreadthIter<'world, Forward> as WorldIter<'world, Q>>::new(self.world)
    }
}