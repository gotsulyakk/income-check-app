slint::include_modules!();
use std::collections::HashMap;

fn get_usd_spendings() -> f64 {
    let mut usd_spendings = HashMap::new();

    usd_spendings.insert("usd_spendings_1", 10);
    usd_spendings.insert("usd_spendings_2", 20);

    // add more spendings here

    let mut total = 0.0;
    for (_, value) in usd_spendings.iter() {
        total += *value as f64;
    }

    total
}

fn get_uah_spendings() -> f64 {
    let mut uah_spendings = HashMap::new();

    uah_spendings.insert("uah_spendings_1", 10_000);
    uah_spendings.insert("uah_spendings_2", 20_000);

    // add more spendings here

    let mut total: f64 = 0.0;
    for (_, value) in uah_spendings.iter() {
        total += *value as f64;
    }

    total
}

fn uah_to_usd(uah_amount: f64, usd_rate: f64) -> f64 {
    uah_amount / usd_rate
}

fn usd_to_uah(usd_amount: f64, usd_rate: f64) -> f64 {
    usd_amount * usd_rate
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |income, usd| {
        let ui = ui_handle.unwrap();
        let income_uah: f64 = income.trim().parse().unwrap();
        let usd_rate: f64 = usd.trim().parse().unwrap();

        let income_usd = uah_to_usd(income_uah, usd_rate);

        let usd_spendings = get_usd_spendings();
        let uah_spendings = get_uah_spendings();

        let usd_spendings_in_uah = usd_to_uah(usd_spendings, usd_rate);
        let uah_spendings_in_usd = uah_to_usd(uah_spendings, usd_rate);

        let total_spendings_uah = usd_spendings_in_uah + uah_spendings;
        let total_spendings_usd = uah_spendings_in_usd + usd_spendings;

        let profit_uah = income_uah - total_spendings_uah;
        let profit_usd = income_usd - total_spendings_usd;

        let result = format!(
            "Дохід у гривнях: {:.2}\nДохід у доларах: {:.2}\nВитрати в гривнях: {:.2}\nВитрати в доларах: {:.2}\nЗалишок у гривнях: {:.2}\nЗалишок у доларах: {:.2}",
            income_uah,
            income_usd,
            total_spendings_uah,
            total_spendings_usd,
            profit_uah,
            profit_usd
        );
        ui.set_results(result.into());
    });

    ui.run()
}
