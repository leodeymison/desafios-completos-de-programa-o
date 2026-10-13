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
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.5, weight: 10.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 50.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 20.0, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Geladeira", area: 3.5, weight: 100.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 50.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Cama boxed", area: 3.0, weight: 100.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 15.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 90.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 34.99, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 56.10, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 80.01, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 50.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.5, weight: 45.77, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 12.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 1.56, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Geladeira", area: 3.5, weight: 2.10, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 23.11, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Cama boxed", area: 3.0, weight: 100.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 34.34, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 149.20, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 73.50, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 43.8, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 9.09, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 50.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.5, weight: 24.20, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 11.11, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 13.9, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Geladeira", area: 3.5, weight: 2.012, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 812.1, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Cama boxed", area: 3.0, weight: 1020.0, city: Cities::PALMAS },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 101.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 91.20, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Fonhas de cama", area: 1.5, weight: 78.20, city: Cities::RIO_DE_JANEIRO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 150.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Boxe Cueca Feminina", area: 1.1, weight: 150.0, city: Cities::SAO_PAULO },
        Product { _id: Uuid::new_v4(), name: "Mesa de jantar", area: 3.0, weight: 50.0, city: Cities::SAO_PAULO },
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

    for logitic in destination.send {
        println!("Caminhão: {} - Q. Produtos: {} - Weight: {}kg - Area: {}³", logitic.truck.name, logitic.products.len(), logitic.truck.weight_max, logitic.truck.area_max);
        println!("Produtos:");
        for product in logitic.products {
            println!("- {}; {}³; {}kg", product.name, product.area, product.weight);
        }
    }

    println!("\n");
    println!("Não enviados:");
    for stop in destination.stop {
        println!(" - {:?}:", stop.city);

        for product in stop.products {
            println!("- {}; {}³; {}kg", product.name, product.area, product.weight);
        }
    }
    println!("\n");
}
