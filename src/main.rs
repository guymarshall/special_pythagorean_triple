/* A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc. */

fn main() {
    let limit: i32 = 1000;

    'outer: for a in 1..limit + 1 {
        for b in 1..limit + 1 {
            for c in 1..limit + 1 {
                let is_triple: bool = (a * a) + (b * b) == (c * c);

                if is_triple {
                    if a + b + c == limit {
                        println!("{} * {} * {} = {}", a, b, c, a * b * c);
                        break 'outer;
                    }
                }
            }
        }
    }
}
