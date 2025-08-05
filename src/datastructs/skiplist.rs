
use rand::Rng;
use std::cmp::Ordering;

/// The node struct of SkipList.
///
/// This struct represents the layout of SkipList's node.
///
/// # Fields
///
/// * `ele` - data stored
/// * `score` - the score for sorting
/// * `backward` - the previous node of current node
/// * `level` - the level entries of SkipList
///
#[derive(Clone)]
pub struct SkipListNode {
    ele: Vec<u8>,
    score: f64,
    backward: Option<Box<SkipListNode>>,
    level: Vec<SkipListLevel>,
}

/// The struct of SkipList's level entry.
///
/// This struct represents the layout of SkipList's level entry.
///
/// # Fields
///
/// * `forward` - the next node of current node in a level
/// * `span` - the gap between current and forward 
///
#[derive(Clone)]
struct SkipListLevel {
    forward: Option<Box<SkipListNode>>,
    span: u32,
}

/// The struct of SkipList.
///
/// This struct represents the layout of SkipList.
///
/// # Fields
///
/// * `header` - the header `SkipListNode` of SkipList
/// * `tail` - the tail `SkipListNode` of SkipList
/// * `length` - the number of nodes in SkipList
/// * `level` - the number of level entries in SkipList
///
#[derive(Clone)]
pub struct SkipList {
    header: Box<SkipListNode>,
    tail: Option<Box<SkipListNode>>,
    length: u64,
    level: i32,
}

const SKIPLIST_MAXLEVEL: u16 = 64;
const SKIPLIST_P: f32 = 0.25;

impl SkipListNode {
    fn random_level() -> u16 {
        let mut lv: u16 = 1;
        let mut rng = rand::rng();

        while rng.random::<u16>() < (SKIPLIST_P * 65535.0) as u16 {
            lv += 1;
        }
        if lv < SKIPLIST_MAXLEVEL {
            return lv;
        }
        return SKIPLIST_MAXLEVEL;
    }

    pub fn new(ele: &[u8], score: f64, backward: Option<Box<SkipListNode>>, lv: u16) -> Self {
        return SkipListNode {ele: ele.to_vec(), score, backward, 
                             level: vec![SkipListLevel{forward: None, span: 0}; lv as usize]};
    }
}

impl SkipList {
    pub fn new() -> Self {
        let header = SkipListNode::new(&[], 0., None, SKIPLIST_MAXLEVEL);
        return SkipList{header: Box::new(header), tail: None, length: 0, level: 1};
    }

    pub fn insert(&mut self, score: f64, ele: &[u8]) -> &SkipListNode {
        let update = [&SkipListNode::new(&[],0., None, 0); SKIPLIST_MAXLEVEL as usize];
        let rank = [0u32; SKIPLIST_MAXLEVEL as usize];
        let x = self.header;
        // let i = 0u16;
        let level = 0u16;

        for i in (0..self.level - 1).rev() {
            rank[i as usize] = if i == self.level -1 {0} else {rank[i as usize - 1]};
            if let Some(node) = x.level[i as usize].forward {
                while node.score < score || (node.score == score && node.ele.cmp(&ele.to_vec()) == Ordering::Less) {
                    rank[i as usize] += x.level[i as usize].span;
                    x = node;
                    if let Some(node) = x.level[i as usize].forward {}
                    else {
                        break;
                    }
                }
                update[i as usize] = &x;
            }
        }

        return todo!();
    }
}