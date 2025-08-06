
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
    span: u64,
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
    level: u16,
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
        let mut update: [Option<&Box<SkipListNode>>; SKIPLIST_MAXLEVEL as usize] = [None; SKIPLIST_MAXLEVEL as usize];
        let mut rank = [0u64; SKIPLIST_MAXLEVEL as usize];
        let x = &self.header;

        for i in (0..self.level as usize).rev() {
            if i == (self.level - 1) as usize {
                rank[i] = 0;
            } else {
                rank[i] = rank[i+1];
            }
            if let Some(_node) = &x.level[i].forward {
                while _node.score < score || (_node.score == score && _node.ele.cmp(&ele.to_vec()) == Ordering::Less) {
                    rank[i] += x.level[i].span;
                    let x = _node;
                    if let Some(_node) = &x.level[i].forward {}
                    else {
                        break;
                    }
                }
                update[i] = Some(x);
            }
        }

        let lv = SkipListNode::random_level();
        if lv > self.level {
            for i  in (self.level as usize..lv as usize) {
                rank[i] = 0;
                update[i] = Some(&self.header);
                update[i].unwrap().level[i].span = self.length;
            }
        }
        return todo!();
    }
}