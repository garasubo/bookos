#![no_std]

use core::ptr::NonNull;
use core::marker::PhantomData;

pub struct ListItem<'a, T> {
    value: T,
    next: Option<NonNull<ListItem<'a, T>>>,
    marker: PhantomData<&'a ListItem<'a, T>>,
}

pub struct LinkedList<'a, T> {
    head: Option<NonNull<ListItem<'a, T>>>,
    last: Option<NonNull<ListItem<'a, T>>>,
    marker: PhantomData<&'a ListItem<'a, T>>,
}

use core::ops::{Deref, DerefMut};

impl<'a, T> Deref for ListItem<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> DerefMut for ListItem<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'a, T> ListItem<'a, T> {
    pub fn new(value: T) -> Self {
        ListItem {
            value,
            next: None,
            marker: PhantomData,
        }
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            last: None,
            marker: PhantomData,
        }
    }

    pub fn push(&mut self, item: &'a mut ListItem<'a, T>) {
        let ptr = unsafe { NonNull::new_unchecked(item as *mut ListItem<T>) };
        let prev_last = self.last.replace(ptr);

        if prev_last.is_none() {
            self.head = Some(ptr);
        } else {
            prev_last.map(|mut i| unsafe {
                i.as_mut().next = Some(ptr);
            });
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn head_mut(&mut self)-> Option<&mut T> {
        self.head.map(|ptr| unsafe { &mut *ptr.as_ptr() }.deref_mut())
    }

    pub fn pop(&mut self) -> Option<&'a mut ListItem<'a, T>> {
        let result = self.head.take();
        let next = result.and_then(|mut ptr| unsafe {
            ptr.as_mut().next
        });

        if next.is_none() {
            self.last = None;
        }

        self.head = next;

        result.map(|ptr| unsafe { &mut *ptr.as_ptr() })
    }
}

#[cfg(test)]
mod test {
    use ListItem;
    use LinkedList;

    #[test]
    fn test_list() {
        let mut item1 = ListItem::new(1);
        let mut item2 = ListItem::new(2);
        let mut item3 = ListItem::new(3);
        let mut list = LinkedList::new();

        list.push(&mut item1);
        list.push(&mut item2);
        list.push(&mut item3);

        assert_eq!(Some(&mut 1), list.head_mut());
        let result1: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 2), list.head_mut());
        let result2: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 3), list.head_mut());
        let result3: &u32 = list.pop().unwrap();
        assert_eq!(1, *result1);
        assert_eq!(2, *result2);
        assert_eq!(3, *result3);

        assert!(list.is_empty());

        let mut item4 = ListItem::new(4);
        let mut item5 = ListItem::new(5);
        list.push(&mut item4);
        list.push(&mut item5);

        let result4: &u32 = list.pop().unwrap();
        let result5: &u32 = list.pop().unwrap();
        assert_eq!(4, *result4);
        assert_eq!(5, *result5);

        assert!(list.is_empty());
    }
}