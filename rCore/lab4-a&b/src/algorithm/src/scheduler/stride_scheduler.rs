//! 最高响应比优先算法的调度器 [`HrrnScheduler`]

use super::Scheduler;
use alloc::collections::LinkedList;
use core::cmp::Ordering;
/// 将线程和调度信息打包

#[derive(Eq)]
struct StrideThread<ThreadType: Clone + Eq> {
    /// 走过的路
    pass: usize,
    /// 步长
    stride: usize,
    /// 线程数据
    pub thread: ThreadType,
}
//大顶堆，所以pass小的优先级高
//pass相同的时候stride小的优先级高

impl <T:Clone+Eq> Ord for StrideThread<T>{
    fn cmp(&self, other: &Self) -> Ordering{
        let sp: usize = self.pass;
        let ss: usize = self.stride;
        let os: usize = other.stride;
        let op: usize = other.pass;    
        if sp == op{
            if ss == os{
                Ordering::Equal
            }
            else if ((ss - os)as isize) > 0{
                Ordering::Less
            }
            else {
                Ordering::Greater
            }
        }
        else if sp  < op{
            Ordering::Greater
        }
        else{
            Ordering::Less
        }

    }
}

impl <T:Clone+Eq> PartialOrd for StrideThread<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sp: usize = self.pass;
        let ss: usize = self.stride;
        let os: usize = other.stride;
        let op: usize = other.pass;
        
        if sp == op{
            if ss == os{
                Some(Ordering::Equal)
            }
            else if ((ss - os)as isize) > 0{
               Some(Ordering::Less)
            }
            else if ((ss - os) as isize) < 0{
                Some(Ordering::Greater)
            }
            else{
                None
            }
        }
        else if ((sp - op) as isize) < 0{
            Some(Ordering::Greater)
        }
        else{
            Some(Ordering::Less)
        }
           
    }
    
}

impl  <T:Clone+Eq> PartialEq for StrideThread<T>  {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass && self.stride == other.stride
    }
}

/// 采用 HRRN（最高响应比优先算法）的调度器
pub struct StrideScheduler<ThreadType: Clone + Eq> {
    MAX_STRIDE : usize, 
    /// 带有调度信息的线程池
    pool: LinkedList<StrideThread<ThreadType>>,
}

// /// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            MAX_STRIDE : 100000 as usize,
            pool: LinkedList::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    type Priority = usize;

    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.push_back(StrideThread {
            pass: 0,
            stride: self.MAX_STRIDE,
            thread,
        })
    }
    
    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(best) = self.pool.iter_mut().max_by(|x, y| {
            (x.cmp(y))
        }) {
            best.pass += best.stride;
            panic!("hello");
            Some(best.thread.clone())
        }else{
            None
        }
        
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut removed = self.pool.drain_filter(|t| t.thread == *thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
        for thd in self.pool.iter_mut(){
            if (*thd).thread == thread{
                (*thd).stride = self.MAX_STRIDE / priority;
            }
        }
    }
}
