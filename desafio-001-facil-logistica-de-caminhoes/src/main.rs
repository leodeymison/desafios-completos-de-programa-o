use std::println;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;

#[derive(Copy, Clone, EnumIter, PartialEq, Debug)]
pub enum Cities {
    PALMAS,
    ARAGUAINA,
    SAO_PAULO,
    RIO_DE_JANEIRO
}

#[derive(Clone, Debug)]
pub struct Truck {
    pub name: &'static str,
    pub area_max: f32,
    pub weight_max: f32,
}
#[derive(Clone, Debug)]
pub struct Product {
    pub _id: Uuid,
    pub name: &'static str,
    pub area: f32,
    pub weight: f32,
    pub city: Cities
}
#[derive(Clone, Debug)]
struct ProductInCities {
    pub products: Vec<Product>,
    pub city: Cities
}

struct SendProductInTruck {
    pub truck: Truck,
    pub products: Vec<Product>,
    pub city: Cities
}

struct DestinationAllProducts {
    send: Vec<SendProductInTruck>,
    stop: Vec<ProductInCities>
}

struct Logistic {
    trucks: Vec<Truck>,
    products: Vec<Product>
}
#[derive(Clone, Debug)]
struct ProductSendAndStop {
    send: Vec<Product>,
    truck: Truck
}

impl Logistic {
    pub fn new(trucks: Vec<Truck>, products: Vec<Product>) -> Logistic {
        Logistic { trucks, products }
    }

    fn product_in_truck_and_stop(truck: &Truck, mut products: Vec<Product>) -> ProductSendAndStop {
        products.sort_by(|a, b| {
            a.area
                .total_cmp(&b.area)
                .then(a.weight.total_cmp(&b.weight))
        });

        let mut result: ProductSendAndStop = ProductSendAndStop { send: vec![], truck: truck.clone() };

        for product in products {
            let count_weight_product_in_truck_current: f32 = result.send.iter().map(|p| p.weight).sum();
            let count_area_product_in_truck_current: f32 = result.send.iter().map(|p| p.area).sum();

            if count_weight_product_in_truck_current + product.weight < truck.weight_max && count_area_product_in_truck_current + product.area < truck.area_max {
                result.send.push(product)
            }
        }

        return result;
    }

    fn remove_product_the_in_list_send(sends: Vec<Product>, list: &mut ProductInCities) {
        let sends_ids: Vec<Uuid> = sends.iter().map(|send| send._id).collect();

        list.products.retain(|product| !sends_ids.contains(&product._id));
    }

    pub fn result(self) -> DestinationAllProducts {
        let mut list_per_cities: Vec<SendProductInTruck> = vec![];

        let mut products_city: Vec<ProductInCities> = vec![];

        for city in Cities::iter() {
            let all_products_in_city: Vec<Product> = self.products.iter().filter(|product| product.city == city).cloned().collect();

            products_city.push(ProductInCities {
                city,
                products: all_products_in_city
            })
        }

        for truck in &self.trucks {
            products_city.sort_by(|a, b| b.products.len().cmp(&a.products.len()));
            
            if !products_city.is_empty() {
                let city_current = &products_city[0];
                let truck_and_stop = Self::product_in_truck_and_stop(truck, city_current.clone().products);

                list_per_cities.push(SendProductInTruck {
                    city: city_current.city,
                    products: truck_and_stop.clone().send,
                    truck: truck.clone()
                });

                Self::remove_product_the_in_list_send(truck_and_stop.send, &mut products_city[0]);
            }
        }

        return DestinationAllProducts { send: list_per_cities, stop: products_city };
    }
}

fn main() {
    let trucks: Vec<Truck> = vec![
        Truck { name: "Alfa One Transnation", area_max: 13.0, weight_max: 1000.0 },
        Truck { name: "Caminão do chico", area_max: 13.0, weight_max: 1000.0 }
    ];
    let products: Vec<Product> = vec![
        Product { _id: Uuid::new_v4(), name: "Cama box casal", area: 2.89, weight: 56.17, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.45, weight: 36.01, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Saco de cimento 50kg", area: 0.19, weight: 43.79, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.4, weight: 41.63, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Fardo de água mineral 12x1,5L", area: 0.1, weight: 16.96, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Fardo de água mineral 12x1,5L", area: 0.11, weight: 16.0, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Fogão 5 bocas", area: 0.35, weight: 33.82, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.22, weight: 18.18, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 14.06, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 11.73, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Saco de ração 15kg", area: 0.21, weight: 15.81, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Cama box casal", area: 2.63, weight: 65.37, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Colchão casal", area: 2.52, weight: 21.69, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.43, weight: 41.31, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.43, weight: 41.31, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Caixa de roupas", area: 0.18, weight: 7.75, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 12.14, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Micro-ondas 30L", area: 0.16, weight: 12.14, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.24, weight: 17.38, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Fardo de arroz 6x5kg", area: 0.24, weight: 33.38, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Fardo de arroz 6x5kg", area: 0.24, weight: 33.38, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Cama box casal", area: 2.75, weight: 59.64, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.45, weight: 46.47, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Kit de ferramentas", area: 0.1, weight: 9.55, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Kit de ferramentas", area: 0.1, weight: 9.55, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Caixa de cerveja 24 latas", area: 0.09, weight: 9.7, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Colchão casal", area: 2.76, weight: 26.79, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Colchão casal", area: 2.76, weight: 26.79, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Geladeira duplex 480L", area: 0.6, weight: 74.62, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.35, weight: 9.4, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Caixa de roupas", area: 0.17, weight: 7.25, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.39, weight: 39.79, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.23, weight: 19.74, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Kit de ferramentas", area: 0.1, weight: 10.53, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar 6 lugares", area: 1.49, weight: 42.43, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Botijão de gás 13kg", area: 0.11, weight: 27.87, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Cadeira estofada", area: 0.24, weight: 5.81, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Cadeira estofada", area: 0.24, weight: 5.81, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Saco de ração 15kg", area: 0.22, weight: 17.04, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Bicicleta aro 29", area: 0.92, weight: 15.72, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Notebook", area: 0.08, weight: 2.29, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Fogão 5 bocas", area: 0.33, weight: 43.53, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Ar-condicionado split 12000 BTUs", area: 0.28, weight: 35.03, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Geladeira duplex 480L", area: 0.56, weight: 78.86, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Geladeira duplex 480L", area: 0.56, weight: 78.86, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Saco de ração 15kg", area: 0.2, weight: 16.68, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.23, weight: 20.23, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.23, weight: 20.23, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.35, weight: 9.61, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.38, weight: 7.62, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Caixa de roupas", area: 0.17, weight: 6.97, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Saco de cimento 50kg", area: 0.18, weight: 56.76, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.23, weight: 20.01, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Caixa de cerveja 24 latas", area: 0.1, weight: 9.74, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Saco de cimento 50kg", area: 0.18, weight: 56.76, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Smart TV 55\"", area: 0.23, weight: 20.01, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Caixa de cerveja 24 latas", area: 0.1, weight: 9.74, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Saco de cimento 50kg", area: 0.19, weight: 45.01, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Fardo de arroz 6x5kg", area: 0.24, weight: 30.02, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Saco de ração 15kg", area: 0.23, weight: 17.18, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar 6 lugares", area: 1.7, weight: 50.38, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Sofá retrátil 3 lugares", area: 1.91, weight: 59.31, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Geladeira duplex 480L", area: 0.58, weight: 77.35, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Kit de ferramentas", area: 0.1, weight: 10.49, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Kit de ferramentas", area: 0.1, weight: 10.49, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Guarda-roupa 6 portas", area: 1.01, weight: 98.63, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Sofá retrátil 3 lugares", area: 1.84, weight: 61.68, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.41, weight: 8.29, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Botijão de gás 13kg", area: 0.11, weight: 27.06, city: Cities::ARAGUAINA },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.41, weight: 8.53, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Pneu aro 15", area: 0.41, weight: 8.53, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Máquina de lavar 12kg", area: 0.45, weight: 46.8, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Bicicleta aro 29", area: 0.83, weight: 16.44, city: Cities::ARAGUAINA },
        
    ];

    println!("\n");
    println!("Produtos totais: {}", &products.len());
    let size_p: f32 = products.iter().map(|p| p.area).sum();
    let weight_p: f32 = products.iter().map(|p| p.weight).sum();
    println!("Produtos size²: {}", size_p);
    println!("Produtos weight: {}", weight_p);

    println!("\n");

    println!("Caminhões totais: {}", &trucks.len());
    let size: f32 = trucks.iter().map(|t| t.area_max).sum();
    let weight: f32 = trucks.iter().map(|t| t.weight_max).sum();
    println!("Caminhões size²: {}", size);
    println!("Caminhões weight: {}", weight);
    println!("\n");

    let logistics: Logistic = Logistic::new(trucks, products);

    let destination = logistics.result();

    println!("\n");
    println!("### ENVIADOS:");
    for logitic in destination.send {
        println!("Caminhão: {} - Q. Produtos: {} - Weight: {}kg - Area: {}³", logitic.truck.name, logitic.products.len(), logitic.truck.weight_max, logitic.truck.area_max);
        let area_p: f32 = logitic.products.iter().map(|p| p.area).sum();
        let weight_p: f32 = logitic.products.iter().map(|p| p.weight).sum();
        println!("Produtos: {:?} - {}kg - {}³", logitic.city, area_p, weight_p);
        for product in logitic.products {
            println!("# {}; {}³; {}kg", product.name, product.area, product.weight);
        }
        println!("\n");
    }

    println!("\n");
    println!("### NÃO ENVIADOS:");
    for stop in destination.stop {
        println!("## {:?}:", stop.city);

        if stop.products.is_empty() {
            println!("# Nenhum produto");
        }

        for product in stop.products {
            println!("# {}; {}³; {}kg", product.name, product.area, product.weight);
        }
        println!("\n");
    }
}
