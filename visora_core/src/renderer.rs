use crate::{treecs::{iterators::{QueryIter, WorldIter}, query::{Query, QueryAble}}, Component};


pub trait Renderer{
    type RenderItem: Component;
    type QueryType<'gui>: for<'a> QueryIter<'gui, &'a Self::RenderItem> + for<'a> WorldIter<'gui, &'a Self::RenderItem>;
    fn render<'gui>(&mut self, q: Query<'gui, &Self::RenderItem, Self::QueryType<'gui>>)
    ;
}