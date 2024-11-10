
#[derive(Debug, Clone)]
pub struct AVLNode<T> {
    pub key: T,
    height: i32,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord + Copy> AVLNode<T> {
    pub fn new(key: T) -> Self {
        AVLNode {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<AVLNode<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(Self::height(&self.left), Self::height(&self.right));
    }

    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().expect("rotate_right called on a node with no left child");
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(self);
        new_root.update_height();
        new_root
    }

    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.right.take().expect("rotate_left called on a node with no right child");
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(self);
        new_root.update_height();
        new_root
    }

    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    fn balance(mut self: Box<Self>) -> Box<Self> {
        self.update_height();
        let balance_factor = self.balance_factor();
        if balance_factor > 1 {
            if self.left.as_ref().map_or(0, |n| n.balance_factor()) < 0 {
                self.left = Some(self.left.take().unwrap().rotate_left());
            }
            self.rotate_right()
        } else if balance_factor < -1 {
            if self.right.as_ref().map_or(0, |n| n.balance_factor()) > 0 {
                self.right = Some(self.right.take().unwrap().rotate_right());
            }
            self.rotate_left()
        } else {
            self
        }
    }

    pub fn insert(mut self: Box<Self>, key: T) -> Box<Self> {
        if key < self.key {
            if let Some(left) = self.left.take() {
                self.left = Some(left.insert(key));
            } else {
                self.left = Some(Box::new(Self::new(key)));
            }
        } else if key > self.key {
            if let Some(right) = self.right.take() {
                self.right = Some(right.insert(key));
            } else {
                self.right = Some(Box::new(Self::new(key)));
            }
        }
        self.balance()
    }

    pub fn find(&self, key: T) -> Option<&AVLNode<T>> {
        if key < self.key {
            self.left.as_ref()?.find(key)
        } else if key > self.key {
            self.right.as_ref()?.find(key)
        } else {
            Some(self)
        }
    }

}