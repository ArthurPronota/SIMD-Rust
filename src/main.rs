#[unsafe(no_mangle)]    // не калечить имя функции
#[inline(never)]    // атрибут, который запрещает компилятору встраивать (инлайнить) функцию в места её вызова.
fn add_arrays(a: &[f32], b: &[f32], out: &mut[f32]) {
    assert_eq!(a.len(), out.len()) ;
    assert_eq!(b.len(), out.len()) ;

    for i in 0..a.len() {
        out[i] = a[i] + b[i] ;
    }
}

fn main() {

    let a = vec![1.0f32; 1000] ;
    let b = vec![2.0f32; 1000] ;
    let mut c = vec![0.0f32; 1000] ;

    add_arrays(&a, &b, &mut c);

    println!("c: {:?}", c) ;
}
