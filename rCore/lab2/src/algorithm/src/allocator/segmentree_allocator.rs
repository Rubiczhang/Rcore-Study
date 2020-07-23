//! 提供线段树结构实现的分配器 [`StackedAllocator`]

use super::Allocator;
use alloc::{vec, vec::Vec};

/// 使用线段树结构实现分配器
///

//位图的掩码
static mask: [u8; 9] = 
    [0, 1, 1<<1, 1 << 2, 1 << 3, 1 << 4, 1 <<5, 1 << 6, 1 << 7];

pub struct SegmentreeAllocator {
    list: SegmentTree,
}

impl Allocator for SegmentreeAllocator {
    fn new(capacity: usize) -> Self {
        let list = SegmentTree::new(capacity);
        Self {
            list
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        self.list.pop()
    }

    fn dealloc(&mut self, index: usize) {
    //     self.list.push((index, index + 1));
        self.list.push(index);
    }
}

struct SegmentTree{
    st: bitMap,
    capacity: usize,
}

impl SegmentTree{
    fn new(capacity: usize) -> Self{
        Self{
            st: bitMap::new(capacity),
            capacity,
        }
    }
    #[inline(always)]
    fn ls(p: usize)->usize{
        p << 1
    }
    #[inline(always)]
    fn rs(p: usize)-> usize{
        (p << 1)+1
    }
    
    fn push_up(&mut self, p: usize){
        if self.st.get(Self::ls(p)) == true && self.st.get(Self::rs(p)) == true{
            self.st.set(p);
        }
    }

    fn push_down(&mut self, p:usize){
        if self.st.get(p){
            self.st.set(Self::ls(p));
            self.st.set(Self::rs(p));
        }
    }

    fn update(&mut self, u: usize, l: usize, r: usize, p: usize, k: bool){
        if u == l && u == r {
            if k {
                self.st.set(p);
            }
            else{
                self.st.clean(p);
            }
        }
        self.push_down(p);
        let m = (l+r) / 2;
        if u <= m  {
            self.update(u, l, m, Self::ls(p), k);
        }
        if u > m{
            self.update(u, m+1, r, Self::rs(p), k);
        }
        self.push_up(p);
    }
    //如果st[q]是真，则返回真
    fn query(&mut self, q: usize, l: usize, r: usize, p: usize)->bool{
        if l <= q && q <= r && self.st.get(p){
            return true
        }
        self.push_down(p);
        let mut ans = false;
        let m = (l+r) / 2;
        if q <= m && self.st.get(p) == false{
            ans = self.query(q, l, m, Self::ls(p));
        }
        if q > m && self.st.get(p) == false{
            ans = self.query(q, m+1, r, Self::rs(p));
        }
        ans
    }
    //分出去一个空栈帧
    fn pop(&mut self)->Option<usize>{
        let mut m: usize = (1 + self.capacity) / 2; //m是可以用的，最小的块
        let mut l: usize = 1;
        let mut r = self.capacity;
        let mut p = 1;
        while l != r{                 //二分查找
            if self.query(m, l, r, p){
                return None
            }
            else{
                //let nm = (l + r) / 2;
                if self.query((l+m)/2, l, m, p) == false{
                    r = m;
                    m = (l + r) / 2;
                    p = Self::ls(p);
                }
                else{
                    l = m+1;
                    m = (l + r) / 2;
                    p = Self::rs(p);
                }
            } 
        }
        self.update(m, 1, self.capacity, 1, true);
        Some(m)
    }

    //把第x个栈帧归还
    fn push(&mut self, x: usize){
        self.update(x, 1, self.capacity, 1, false);
    }
}

struct bitMap{
    bm: Vec<u8>,
}

impl bitMap{
    fn new(capacity: usize) -> Self{
        let bm = vec![0 as u8; capacity * 4];
        Self{
            bm,
        }
    }

    fn set(&mut self, x: usize){
        let id = x / 8;
        let m = x % 8 + 1;
        self.bm[id] = self.bm[id] | mask[m];
    }

    fn clean(&mut self, x: usize){
        let id = x / 8;
        let m = x % 8+1;
        self.bm[id] = self.bm[id] & (mask[m] ^ (0xff));
    }

    fn get(&self, x: usize) -> bool{
        let id = x / 8;
        let m = x % 8+1;
        let res = self.bm[id] & mask[m];
        res != 0
    }

}
