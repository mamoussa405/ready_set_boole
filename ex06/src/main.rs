use ex06;

fn main() {
    println!("{}", ex06::conjunctive_normal_form("AB=B="));
    ex06::conjunctive_normal_form("XY=VK>=L&E^G|");
}