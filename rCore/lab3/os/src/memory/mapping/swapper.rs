//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use bitflags::*;

/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = ClockSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}

pub struct ClockSwapper{
    queue: Vec<(VirtualPageNumber, FrameTracker, usize)>,
    quota: usize,
    current_node:  usize,
}

impl Swapper for ClockSwapper{
    fn new(quota: usize) -> Self{
        Self{
            queue: Vec::new(),
            quota,
            current_node: 0,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>{
        unsafe{
            //遍历循环列表，注意len()+1的作用
            for i in 0..(self.queue.len()+1){
                //1<<6是entry的访问位
                self.current_node = (self.current_node+1) % self.quota;
                let cn = self.current_node;
                let pe = self.queue[cn].2 as * mut PageTableEntry;
                let mut flags = (*pe).flags().clone();
                if flags.contains(Flags::ACCESSED){
                    flags.set(Flags::ACCESSED, false);
                    (*pe).set_flags(flags);
                }
                else{
                    //找到合适的页面
                    break;
                }
            }
        }
        //理论上说不用remove出去 ，只要把值返回就好了
        // 但是FramTracker又没有实现Copy，直接返回过不了编译器
        let res = self.queue.remove(self.current_node);
        //Some((self.queue[self.current_node].0, self.queue[self.current_node].1))
        //这里直接访问会编译不过，但是删除之后用这个值在返回就不会报错，很奇怪
        Some((res.0, res.1))
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        let cn = self.current_node;
        //本来不用这么麻烦的 这样实现的话复杂度为O（n） 
        //但是由于水平不够 不知道怎么返回才合法 所以只能这样了
        self.queue.insert(cn, (vpn, frame, entry as usize));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}

