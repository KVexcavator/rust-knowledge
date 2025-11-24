fn display_neutron_flow(polarity: isize) {
    println!(
        "Neutron flow is {}",
        if polarity < 0 {"reversed"} else {"normal"}
    );
}

fn main() {
    let polarity = 1;
    {
        let polarity = polarity - 2;
        display_neutron_flow(polarity);
    }
    display_neutron_flow(polarity);
}
/*
вернет
Neutron flow is reversed
Neutron flow is normal

Повторное использование имён переменных известно как затенение и является спорной темой среди разработчиков Rust. Rust явно разрешает создание затенённых переменных; однако, в отличие от многих других языков, вы не увидите предупреждения компилятора при использовании этой возможности.
После того, как вы затенили переменную, вы не сможете получить доступ к исходной переменной, пока новая привязка не выйдет из области действия.
 */
