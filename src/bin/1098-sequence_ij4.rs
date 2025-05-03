fn main()
{
    let mut i: f64 = 0.0;

    while i <= 2.0 + 1e-9
    {
        let mut j = 1.0;
        while j <= 3.0
        {
            if (i - 0.0).abs() < 1e-9 || 
               (i - 1.0).abs() < 1e-9 || 
               (i - 2.0).abs() < 1e-9
            {
                println!("I={} J={}", i as i32, (j + i) as i32);
            }
            else
            {
                println!("I={:.1} J={:.1}", i, j + i);
            }
            j += 1.0;
        }
        i += 0.2;
        i = (i * 10.0).round() / 10.0;
    }
}
