use cbuffer::solution::CircularBuffer;

#[test]
pub fn test_write_size(){
    let mut b = CircularBuffer::new(1);
    b.write(5).unwrap();
    assert_eq!(1, b.size());
}

#[test]
pub fn test_write_read(){
    let mut b = CircularBuffer::new(1);
    b.write(5).unwrap();
    assert_eq!(b.size, 1);
    assert_eq!(b.read(), Some(5));
}

#[test]
pub fn test_sequence(){
    let mut b = CircularBuffer::new(3);
    b.write(5).unwrap();
    b.write(4).unwrap();
    b.write(3).unwrap();
    assert_eq!(5, b.read().unwrap());
    assert_eq!(4, b.read().unwrap());
    assert_eq!(3, b.read().unwrap());
}

#[test]
pub fn test_wrap_around(){
    let mut b = CircularBuffer::new(3);
    // 5 4 3, tail ritorna a 0 e size è 3
    b.write(5).unwrap();
    b.write(4).unwrap();
    b.write(3).unwrap();
    
    // leggo 5 , head a 1 , leggo 4 , head a 2
    b.read().unwrap();
    b.read().unwrap();
    
    // scrivo 3 in tail 0, tail a 1
    // scrivo 4 in tail 1, tail a 2
    // size a 3
    b.write(3).unwrap();
    b.write(4).unwrap();
    
    // alla fine avrò 3 4 3
    assert_eq!(b.read(),Some(3));
    assert_eq!(b.read(),Some(3));
    assert_eq!(b.read(),Some(4));

}

#[test]
pub fn test_read_empty(){
    let mut b = CircularBuffer::new(3);
    assert_eq!(b.read(), None);

    b.write(10).unwrap();
    b.read().unwrap();
    assert_eq!(b.read(), None);
}

#[test]
pub fn test_write_full(){
    let mut b = CircularBuffer::new(3);
    b.write(10).unwrap();
    b.write(9).unwrap();
    b.write(8).unwrap();

    assert!(b.write(10).is_err());
}

#[test]
pub fn test_overwrite(){
    let mut b = CircularBuffer::new(2);
    b.write(1).unwrap();
    b.write(2).unwrap();

    // Sovrascriviamo l'1 (il più vecchio) con il 3
    b.overwrite(3);

    // Ora il primo elemento letto deve essere il 2, poi il 3
    assert_eq!(b.read(), Some(2));
    assert_eq!(b.read(), Some(3));
    assert_eq!(b.read(), None);
}

#[test]
pub fn test_make_contiguous(){
    let mut b = CircularBuffer::new(4);
    b.write(1).unwrap();
    b.write(2).unwrap();
    b.read().unwrap();
    b.write(3).unwrap();
    b.write(4).unwrap();
    b.write(5).unwrap(); // Qui il buffer è "spezzato" (wrap)

    b.make_contiguous();

    // L'ordine logico deve essere preservato: 2, 3, 4, 5
    assert_eq!(b.read(), Some(2));
    assert_eq!(b.read(), Some(3));
    assert_eq!(b.read(), Some(4));
    assert_eq!(b.read(), Some(5));
}
