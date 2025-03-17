fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total_weight: u32 = shipments.iter().sum();
    let ship_count = shipments.len();

    if total_weight % ship_count as u32 != 0 {
        return usize::MAX;
    }

    let target_weight = total_weight / ship_count as u32;
    let mut moves = 0;
    let mut excess_weight = 0;

    for &shipment in shipments {
        excess_weight += shipment as i32 - target_weight as i32;
        moves += excess_weight.abs() as usize;
    }

    moves
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let result1 = count_permutation(&shipments1);
    if result1 == usize::MAX {
        println!("Неможливо рівномірно розподілити вантаж.");
    } else {
        println!("Мінімальна кількість переносу вантажу: {}", result1);
    }

    let shipments2 = vec![9, 3, 7, 2, 9];
    let result2 = count_permutation(&shipments2);
    if result2 == usize::MAX {
        println!("Неможливо рівномірно розподілити вантаж.");
    } else {
        println!("Мінімальна кількість переносу вантажу: {}", result2);
    }
}
