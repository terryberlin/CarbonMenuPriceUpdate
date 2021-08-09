#![allow(unused_variables)]
#![allow(unused_macros)]

use adjunct::{
    Category, DiscountAmount, DiscountDefinition, ItemDefinition, ItemQuantityConstraint,
    ItemSelection, Menu, Modification, OrderConstraint, OrderTimeConstraint, OrderTotalConstraint,
    PriceOverride, PricingModification, PricingModificationStyle, PricingRule, PricingRuleSet,
    SlotDefinition, SlotType, Variation, ID,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut menu = Menu::default();
    macro_rules! item {
        ($($element: ident: $val: expr),*) => {
            {
                let item = ItemDefinition { $($element: $val.into()),*, ..ItemDefinition::default() };
                menu.items.push(item.clone());
                item
            }
        }
    }

    macro_rules! slot {
        ($($element: ident: $val: expr),*) => {
            {
                let slot = SlotDefinition { $($element: $val.into()),*, ..SlotDefinition::default() };
                slot
            }
        }
    }

    macro_rules! root_slot {
        ($($element: ident: $val: expr),*) => {
            {
                let slot = SlotDefinition { $($element: $val.into()),*, ..SlotDefinition::default() };
                menu.slots.push(slot.clone());
                slot
            }
        }
    }

    // Used for data stored in a hashset (Tags, item_ids, etc.)
    macro_rules! hashset{
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = BTreeSet::new();  // Create a mutable Set
                $(
                    temp_set.insert($x.into()); // Insert each item matched into the HashSet
                )*
                temp_set // Return the populated HashSet
            }
        };
    }

    macro_rules! modifier_upcharge {
        ( $b:expr; $($x:expr => $y:expr),* ) => ({
            let mut temp_map = BTreeMap::with_b($b);
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        });
        ( $($x:expr => $y:expr),* ) => ({
            let mut temp_map = BTreeMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        });
        ( $b:expr; $($x:expr => $y:expr,)* ) => (
            btreemap!{$b; $($x => $y),*}
        );
        ( $($x:expr => $y:expr,)* ) => (
            btreemap!{$($x => $y),*}
        );
    }

    macro_rules! variation {
        ($($element: ident: $val: expr),*) => {
            {
                let variation = Variation { $($element: $val.into()),*, id: ID::new_v4(), ..Variation::default() };

                variation
            }
        }
    }

    macro_rules! price_override {
        ($($element: ident: $val: expr),*) => {
            {
                let p_override = PriceOverride { $($element: $val.into()),*, ..PriceOverride::default() };

                p_override
            }
        }
    }

    macro_rules! category {
        ($($element: ident: $val: expr), *) => {
            {
                let category = Category { $($element: $val.into()), *, ..Category::default()};
                menu.categories.push(category.clone());
                category
            }
        }
    }

    macro_rules! discount {
        ($($element: ident: $val: expr), *) => {
            {
                let discount = DiscountDefinition { $($element: $val.into()), *, ..DiscountDefinition::default() };
                menu.discounts.push(discount.clone());
                discount
            }
        }
    }

    macro_rules! dynamic_price {
        ($($element: ident: $val: expr), *) => {
            {
                let dp = PricingRuleSet { $($element: $val.into()), *, ..PricingRuleSet::default() };
                menu.dynamic_pricing.push(dp.clone());
                dp
            }
        }
    }

    macro_rules! pricing_rule {
        ($($element: ident: $val: expr), *) => {
            {
                PricingRule { $($element: $val.into()), *, ..PricingRule::default() }
            }
        }
    }

    macro_rules! pricing_modification {
        ($($element: ident: $val: expr), *) => {
            {
                PricingModification { $($element: $val.into()), *, ..PricingModification::default() }
            }
        }
    }

    // Categories
    let __category_quick_pickss = category!(name: "Quick Picks", pos_menu: true, tags: hashset!("quick"), image: Some(String::from("quick_picks.png")));
    let __category_combos = category!(name: "Combos", pos_menu: true, tags: hashset!("combos"), image: Some(String::from("combos.png")));
    let _category_tacos = category!(name: "Tacos", pos_menu: true, tags: hashset!("tacos"), image: Some(String::from("tacos.png")));
    let _category_burritos = category!(name: "Burritos", pos_menu: true, tags: hashset!("burrito"), image: Some(String::from("burritos.png")));
    let _category_specialties = category!(name: "Specialties", pos_menu: true, tags: hashset!("specialties"), image: Some(String::from("specialties.png")));
    let _category_breakfast = category!(name: "Breakfast", pos_menu: true, tags: hashset!("breakfast", "breakfast_combo"), image: Some(String::from("breakfast.png")));
    let _category_sides = category!(name: "Sides", pos_menu: true, tags: hashset!("sides"), image: Some(String::from("sides.png")));
    let _category_drinks = category!(name: "Drinks", pos_menu: true, tags: hashset!("drinks"), image: Some(String::from("breakfast.png")));
    let _category_kids_meal = category!(name: "Kids Meal", pos_menu: true, tags: hashset!("kids_meal"), image: Some(String::from("favorites.png")));
    let _category_value = category!(name: "Valuest", pos_menu: true, tags: hashset!("value"), image: Some(String::from("valuest.png")));
    let _category_limited_time_offer = category!(name: "LTO", pos_menu: true, tags: hashset!("LTO"), image: Some(String::from("limited time.png")));
    let _category_desserts = category!(name: "Desserts", pos_menu: true, tags: hashset!("dessert"), image: Some(String::from("desserts.png")));
    let _category_side_of = category!(name: "Side Of...", pos_menu: true, tags: hashset!("side_of"), image: Some(String::from("limited time.png")), multi_select_modal: true);

    // Proteins
    let sausage = item!(id: ID::parse_str("aa9f634c-3547-4e28-9405-f760927f77f3").unwrap(), long_name: "Chorizo Sausage", short_name: "Saus", price: 109, plu: "999003", tags: hashset!("breakfast_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 109));
    let bacon = item!(id: ID::parse_str("abed8ab2-3d94-4bcf-b2fd-34c698659adc").unwrap(), long_name: "Bacon", short_name: "BA", price: 109, plu: "999002", tags: hashset!("breakfast_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 109));
    let ground_beef = item!(id: ID::parse_str("ac99e1be-e648-4b97-b878-b8e1ca558530").unwrap(), long_name: "Beef", short_name: "BF", price: 69, plu: "999014", tags: hashset!("proteins", "breakfast_proteins", "grilled_burrito_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 69));
    let chicken = item!(id: ID::parse_str("acea615b-8b2c-47ae-b7b7-84eeb8d2ec15").unwrap(), long_name: "Chicken", short_name: "CK", price: 125, plu: "999005", tags: hashset!("proteins", "street_taco_proteins", "grilled_burrito_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 125));
    let steak = item!(id: ID::parse_str("ae3a6089-33c9-43f8-8052-e5b2d54fe410").unwrap(), long_name: "Sirloin Steak", short_name: "STK", price: 159, plu: "6950", tags: hashset!("proteins", "breakfast_proteins", "street_taco_proteins", "grilled_burrito_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 159));
    let fried_chicken = item!(id: ID::parse_str("ae87394f-d805-4034-8870-7ae11905dac6").unwrap(), long_name: "Fried Chicken", short_name: "FC", price: 119, plu: "999153", tags: hashset!("proteins", "grilled_burrito_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 119));
    let shrimp = item!(id: ID::parse_str("aef23599-1f33-412e-aae0-50fda9cc6c38").unwrap(), long_name: "Shrimp", short_name: "SHP", price: 99, plu: "999200",tags: hashset!("street_taco_proteins"), modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 99));
    let fish = item!(id: ID::parse_str("af04bef4-8e1b-417e-b02d-e282a5676f65").unwrap(), long_name: "Fish", short_name: "FSH", price: 99, plu: "999058", modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 99));

    // Sauces
    let mild_sauce = item!(id: ID::parse_str("cde0371e-6e80-4e06-8988-d8084cd52e52").unwrap(), long_name: "Mild Sauce", short_name: "MS", tags: hashset!("sauces") ,modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999015");
    let house_salsa = item!(id: ID::parse_str("ce864edc-60f4-414e-adf9-edc291e55e1d").unwrap(), long_name: "SD House", short_name: "SD HSE", tags: hashset!("sauces") ,modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999092");
    let chimichurri = item!(id: ID::parse_str("d5acba3e-2e53-4b2b-8b61-1dbbc8356663").unwrap(), long_name: "Chimichurri", short_name: "CSC", tags: hashset!("sauces"), modifiers: vec!(Modification::Extra, Modification::Light), price: 59, plu: "999176", modifier_upcharge: modifier_upcharge!(Modification::Extra => 59));
    let creamy_chipotle = item!(id: ID::parse_str("d5e359e9-a5c8-4243-8d39-6f19fecd64ae").unwrap(), long_name: "Creamy Chipotle", short_name: "CCS", tags: hashset!("sauces"), modifiers: vec!(Modification::Extra, Modification::Light), price: 29, plu: "8690", modifier_upcharge: modifier_upcharge!(Modification::Extra => 29));
    let super_hot = item!(id: ID::parse_str("c82e86c8-2962-40cb-96af-456bcdebab33").unwrap(), long_name: "Super Hot", short_name: "SH", modifiers: vec!(Modification::Extra, Modification::Light), plu: "999027");
    let sour_cream = item!(id: ID::parse_str("d8f7a827-94c2-4eac-959d-aa2d260ced28").unwrap(), long_name: "Sour Cream", short_name: "SC",tags: hashset!("sauces",  "sauce_addition"), modifiers: vec!(Modification::Extra, Modification::Light), price: 39, plu: "999025", modifier_upcharge: modifier_upcharge!(Modification::Extra => 39));
    let guacamole = item!(id: ID::parse_str("d93bbcb5-e3b4-4b3a-b638-f87a50d25b5c").unwrap(), long_name: "Guacamole", short_name: "GUAC", tags: hashset!("sauces", "sauce_addition") ,modifiers: vec!(Modification::Extra, Modification::Light), price: 69, plu: "999009", modifier_upcharge: modifier_upcharge!(Modification::Extra => 69));

    // Ingredients
    let cheese = item!(id: ID::parse_str("af9128ce-8984-49c4-ae01-52ba72957cd2").unwrap(), long_name: "Cheese Blend", short_name: "CH BLD", modifiers: vec!(Modification::Extra, Modification::Light), price: 40, plu: "999041", modifier_upcharge: modifier_upcharge!(Modification::Extra => 40));
    let cheddar_cheese = item!(id: ID::parse_str("529726e2-8068-49ea-b6ad-8930c0ec59bf").unwrap(), long_name: "Cheese", short_name: "Cheese", modifiers: vec!(Modification::Extra, Modification::Light), price: 35, plu: "999004", modifier_upcharge: modifier_upcharge!(Modification::Extra => 35));
    let nacho_cheese = item!(id: ID::parse_str("b302bbfc-c6d5-4a76-a2ea-497373b333bc").unwrap(), long_name: "Nacho Cheese", tags: hashset!("sauce_addition", "sauces"), short_name: "Nacho Cheese", modifiers: vec!(Modification::Extra, Modification::Light), price: 59, plu: "999016", modifier_upcharge: modifier_upcharge!(Modification::Extra => 59));
    let scrambled_eggs = item!(id: ID::parse_str("b3b714fa-5356-4746-ac11-df0cd3971c38").unwrap(), long_name: "Eggs", short_name: "Eggs", modifiers: vec!(Modification::Extra, Modification::Light), price: 89, plu: "999008", modifier_upcharge: modifier_upcharge!(Modification::Extra => 89));
    let lettuce = item!(id: ID::parse_str("b3d7660d-a77d-4eb6-baaf-b33435a91fb6").unwrap(), long_name: "Lettuce", short_name: "Lettuce", modifiers: vec!(Modification::Extra, Modification::Light), price: 29, plu: "999013", modifier_upcharge: modifier_upcharge!(Modification::Extra => 29));
    let tomato = item!(id: ID::parse_str("b4010255-11ec-45d3-9dbe-11f52f861893").unwrap(), long_name: "Tomato", short_name: "Tomato", modifiers: vec!(Modification::Extra, Modification::Light), price: 28, plu: "999028", modifier_upcharge: modifier_upcharge!(Modification::Extra => 28));
    let hard_shell = item!(id: ID::parse_str("b46f0def-750c-4d82-b7cf-81509b3e280a").unwrap(), long_name: "Crispy Shell", short_name: "Shell", price: 0, plu: "999052", tags: hashset!("shell"));
    let small_tortilla = item!(id: ID::parse_str("b4a19204-72b6-4cab-a75f-b4efed17d471").unwrap(), long_name: "Tort 6", short_name: "Tort 6", price: 0, plu: "999053", modifiers: vec!(Modification::Custom(String::from("Grilled"))), tags: hashset!("shell"));
    let med_tortilla = item!(id: ID::parse_str("b4bd2f93-cbce-461c-a9dd-da8571866e64").unwrap(), long_name: "Tort 8", short_name: "Tort 8", price: 0, plu: "999156", modifiers: vec!(Modification::Custom(String::from("Grilled"))));
    let large_tortilla = item!(id: ID::parse_str("b55d3b8e-85f2-479e-ba5a-cea85b0fec5b").unwrap(), long_name: "Tort 10", short_name: "Tort 10", price: 0, plu: "999054", modifiers: vec!(Modification::Custom(String::from("Grilled"))));
    let corn_tortilla = item!(id: ID::parse_str("b7cfa800-3b66-4fdd-894a-48b71dfe042a").unwrap(), long_name: "CF Tort", short_name: "CF-TRT", price: 0, plu: "999056");
    let taco_bowl = item!(id: ID::parse_str("b7f8c4d7-3ae9-4299-8306-99e0394668e8").unwrap(), long_name: "Salad Bowl", short_name: "Shell", price: 99, plu: "999039", modifier_upcharge: modifier_upcharge!(Modification::Extra => 99));
    let corn_chips = item!(id: ID::parse_str("b9256a46-c9ba-4370-b828-71a4be94505f").unwrap(), long_name: "Chips", short_name: "Chips", price: 0, plu: "999040");
    let onions = item!(id: ID::parse_str("b9c5cb73-57f2-4e36-baaa-1c137f732b7a").unwrap(), long_name: "Onion", short_name: "Onion", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999019");
    let poblano_peppers = item!(id: ID::parse_str("bbccf37c-b49d-4470-a549-0acd2a56f64d").unwrap(), long_name: "Poblano Peppers", short_name: "PBLANO", modifiers: vec!(Modification::Extra, Modification::Light) ,price: 0, plu: "999207");
    let jalapeno = item!(id: ID::parse_str("c0a7f7ec-3d4f-4b26-b671-b06c4327fb38").unwrap(), long_name: "Jalapenos", short_name: "JAL", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999012");
    let pico_de_gallo = item!(id: ID::parse_str("c1fb6dbe-2117-472b-834b-2e0106cdb3c9").unwrap(), long_name: "Pico de Gallo", short_name: "PICO", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999020");
    let black_beans = item!(id: ID::parse_str("c20ba542-a949-453f-bea8-f5e9198f1d6c").unwrap(), long_name: "Black Beans", short_name: "B BN", modifiers: vec!(Modification::Extra, Modification::Light), price: 49, plu: "999119", modifier_upcharge: modifier_upcharge!(Modification::Extra => 49));
    let refried_beans = item!(id: ID::parse_str("c2457ed3-7500-4e95-b26e-81912d4d055f").unwrap(), long_name: "Beans", short_name: "Beans", modifiers: vec!(Modification::Extra, Modification::Light), price: 28, plu: "999001", tags: hashset!("proteins", "combosides"), modifier_upcharge: modifier_upcharge!(Modification::Extra => 28));
    let potato_ole_seasoning = item!(id: ID::parse_str("c3ee78d6-a2c4-41c6-9107-83ce79a5ddb0").unwrap(), long_name: "Seasoning", short_name: "Seasoning", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "999024");
    let potato_ole = item!(id: ID::parse_str("c4d318dd-0b38-4f5c-9816-bdb2aeafeeda").unwrap(), long_name: "Potato Oles", short_name: "Oles", modifiers: vec!(Modification::Extra, Modification::Light), price: 33, plu: "999017", modifier_upcharge: modifier_upcharge!(Modification::Extra => 33));
    let burger_bun = item!(id: ID::parse_str("c5a24bff-8a77-449c-b69f-8884696e3231").unwrap(), long_name: "Bun", short_name: "BUN", price: 0, plu: "999999");
    let salsa = item!(id: ID::parse_str("c669cbbc-158d-44ad-b1e1-6c9ab97fb5fd").unwrap(), long_name: "Salsa", short_name: "SAL", price: 0, plu: "999023",  modifiers: vec!(Modification::Extra, Modification::Light));
    let cilantro_lime_rice = item!(id: ID::parse_str("c78e89c8-88e5-46f7-8f1f-66ac04569908").unwrap(), long_name: "Cil Lime Rice", short_name: "CLRICE", price: 75, plu: "999062",  modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 75));
    let queso_fresco = item!(id: ID::parse_str("c7d6eb20-746c-4425-bb2b-e3d9538fb3ea").unwrap(), long_name: "Queso Blend Chz", short_name: "Q B CH", price: 29, plu: "999210",tags:hashset!("sauces"),  modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 29));
    let lime = item!(id: ID::parse_str("c8124feb-76f4-44d9-ba05-e468e8ab6510").unwrap(), long_name: "Lime", short_name: "LIME", price: 0, plu: "999151",  modifiers: vec!(Modification::Extra, Modification::Light));
    let ranch = item!(id: ID::parse_str("d28c6bf0-475e-4af2-9a97-6c55a8fc50a8").unwrap(), long_name: "Ranch", short_name: "Ranch", price: 49, plu: "999021",  modifiers: vec!(Modification::Extra, Modification::Light));
    let corn_salsa = item!(id: ID::parse_str("e322859d-a96c-4992-bbc1-7b90589b9a93").unwrap(), long_name: "Corn Salsa", short_name: "Corn Salsa", price: 0, plu: "9996610",  modifiers: vec!(Modification::Extra, Modification::Light));

    // Dessert ingredients
    let cream_cheese_icing = item!(id: ID::parse_str("c897434d-8054-45d9-a966-8b7545dc3d48").unwrap(), long_name: "Glaze Icing", short_name: "GLZ", price: 30, plu: "999400", modifiers: vec!(Modification::Extra, Modification::Light), modifier_upcharge: modifier_upcharge!(Modification::Extra => 30));
    let cinnamon_sugar = item!(id: ID::parse_str("c94e22a2-0205-4c61-945e-461e8e7cc12f").unwrap(), long_name: "Cinn Sugar", short_name: "CINSGR",price: 0, plu: "999032", modifiers: vec!(Modification::Extra, Modification::Light));

    // Drink Ingredients
    let sugar = item!(id: ID::parse_str("ca9dd236-b3e3-4e2a-8c36-dc6c45614efc").unwrap(), long_name: "Sugar", short_name: "SGR", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "3021");
    let cream = item!(id: ID::parse_str("cae7115d-54fc-4a32-84b3-eeb10a2f5fae").unwrap(), long_name: "Cream", short_name: "CRM", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "3022");
    let sweetener = item!(id: ID::parse_str("cb6b8711-13cf-46b2-9624-288f158b641a").unwrap(), long_name: "Sweetener", short_name: "SWT", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "3023");
    let lemon = item!(id: ID::parse_str("cbca854c-6bdc-4de4-813f-6019a947df7b").unwrap(), long_name: "Lemon", short_name: "LMN", modifiers: vec!(Modification::Extra, Modification::Light), price: 0, plu: "6160");
    let cane_syrup = item!(id: ID::parse_str("af7642bc-bc49-4561-815a-c6a8748e4447").unwrap(), long_name: "Cane Syrup", short_name: "Cane", modifiers: vec!(Modification::Extra, Modification::Light), price: 25, plu: "999220", modifier_upcharge: modifier_upcharge!(Modification::Extra => 25));
    let mocha_pump = item!(id: ID::parse_str("a33d3fa9-0953-4622-85f4-ede7b743d487").unwrap(), long_name: "Mocha Pump", short_name: "Mocha", modifiers: vec!(Modification::Extra, Modification::Light), price: 25, plu: "999221", modifier_upcharge: modifier_upcharge!(Modification::Extra => 25));
    let vanilla_pump = item!(id: ID::parse_str("33fc2876-8e76-4189-ba54-8f430e818376").unwrap(), long_name: "Vanilla Pump", short_name: "Vanilla", modifiers: vec!(Modification::Extra, Modification::Light), price: 25, plu: "999222", modifier_upcharge: modifier_upcharge!(Modification::Extra => 25));

    // Dressing
    let bacon_ranch = item!(id: ID::parse_str("d967141e-6d09-4e33-a41d-a31c56cb50fe").unwrap(), long_name: "Bacon Ranch", short_name: "B RD", tags: hashset!("dressing"), price: 89, plu: "8370");
    let blue_cheese = item!(id: ID::parse_str("d9f98c16-0d73-4555-ae5e-3df5bae71075").unwrap(), long_name: "Blue Cheese", short_name: "BLU CH", tags: hashset!("dressing"), price: 89, plu: "8380");
    let fat_free_ranch = item!(id: ID::parse_str("db0abe4c-56a4-467d-b45d-f2e935212dbf").unwrap(), long_name: "Fat Free Ranch", short_name: "FF Ranch", tags: hashset!("dressing"), price: 89, plu: "8430");
    let thousand_island = item!(id: ID::parse_str("db0b323c-0861-44e4-8c28-75289df0efc3").unwrap(), long_name: "1000", short_name: "1000", tags: hashset!("dressing"), price: 55, plu: "8500");
    let fat_free_thousand_island = item!(id: ID::parse_str("de1ec281-d213-46ee-adc9-e7d9690d116c").unwrap(), long_name: "FF 1000", short_name: "FF 1000", tags: hashset!("dressing"), price: 55, plu: "8510");
    let italian = item!(id: ID::parse_str("df7675a6-719e-4610-825a-e51428e6b6e9").unwrap(), long_name: "Italian", short_name: "ITAL", tags: hashset!("dressing"), price: 55, plu: "8520");
    let light_italian = item!(id: ID::parse_str("df95d5c9-4273-4b4f-bd7d-0cdc4b35e78d").unwrap(), long_name: "Light Italian", short_name: "LT ITAL", tags: hashset!("dressing"), price: 55, plu: "8530");
    let fat_free_french = item!(id: ID::parse_str("e09660ea-a6bc-4cc0-909c-79c5f7f434b2").unwrap(), long_name: "FF Frn", short_name: "FF FRN", tags: hashset!("dressing"), price: 55, plu: "8540");
    let french = item!(id: ID::parse_str("e11a0ecd-c401-4f75-bf67-5588649995d2").unwrap(), long_name: "French", short_name: "French", tags: hashset!("dressing"), price: 55, plu: "8570");
    let house = item!(id: ID::parse_str("e258fb85-a588-4e10-a3a1-2d97365c01c8").unwrap(), long_name: "Side House", short_name: "HD", tags: hashset!("dressing"), price: 99, plu: "8440");
    let side_ranch = item!(id: ID::parse_str("e387bbb1-ab6c-435b-b7d3-780e86f6b661").unwrap(), long_name: "Side Ranch", short_name: "RAN", tags: hashset!("dressing"), price: 99, plu: "8390");
    let jalapeno_ranch = item!(id: ID::parse_str("e4573965-7f06-487a-8fbd-393619402a34").unwrap(), long_name: "Jalapeno Ranch", short_name: "SD JR", tags: hashset!("dressing"), price: 49, plu: "12600");
    let chipotle_lime = item!(id: ID::parse_str("e6eb880c-a232-4966-beb7-2b9cd3fb96e2").unwrap(), long_name: "Chipotle Lime", short_name: "SD CLS", tags: hashset!("dressing"), price: 49, plu: "12610");

    let super_size_variations = vec![
        variation!( name: "Small", prefix: Some(String::from("SM")), identifier: "128"),
        variation!( name: "Medium", prefix: Some(String::from("MD")), identifier: "256"),
    ];

    let taco_proteins = slot!(
        name: "Protein",
        slot_type: SlotType::Ingredient,
        selection: ItemSelection::Tag("proteins".to_owned()),
        default_item_ids: vec!(ground_beef.id)
    );
    let grilled_burrito_proteins = slot!(
        name: "Protein",
        slot_type: SlotType::Ingredient,
        selection: ItemSelection::Tag("grilled_burrito_proteins".to_owned()),
        default_item_ids: vec!(chicken.id),
        price_overrides: vec!(price_override!(tags: hashset!("grilled_burrito_proteins"), price: 0))
    );
    let street_taco_proteins = slot!(
        name: "Protein",
        slot_type: SlotType::Ingredient,
        selection: ItemSelection::Tag("street_taco_proteins".to_owned()),
        default_item_ids: vec!(steak.id),
        price_overrides: vec!(price_override!(tags: hashset!("street_taco_proteins"), price: 0))
    );

    let breakfast_proteins = slot!(
        name: "Protein",
        slot_type: SlotType::Replace,
        selection: ItemSelection::Tag("breakfast_proteins".to_owned()),
        price_overrides: vec!(price_override!(tags: hashset!("breakfast_proteins"), price: 0))
    );

    // Sides

    let side_variations = vec![
        variation!( name: "Kid", prefix: Some(String::from("KD")), identifier: "64", price: Some(100)),
        variation!( name: "Small", prefix: Some(String::from("SM")), identifier: "128", price: Some(230)),
        variation!( name: "Medium", prefix: Some(String::from("MD")), identifier: "256", price: Some(289)),
        variation!( name: "Large", prefix: Some(String::from("LG")), identifier: "512", price: Some(359)),
    ];

    let sauces = slot!(
        name: "Sauces",
        slot_type: SlotType::Items,
        collapsed: false,
        selection: ItemSelection::Tag("sauces".to_owned()),
        price_overrides: vec!(price_override!(tags: hashset!("sauces"), price: 0)),
        default_item_ids: vec!(
            mild_sauce.id
        )
    );

    let potato_ole_side = item!(
        id: ID::parse_str("051aba55-b9f0-418b-a6e4-623f7992b28f").unwrap(),
        long_name: "Potato Oles",
        short_name: "PO",
        price: 0, plu: "1830",
        tags: hashset!("sides", "combosides"),
        variations: side_variations.clone(),
        item_priority: 2,
        modifiers: vec!(Modification::Custom("Well Done".to_string())),
        slots: vec!(slot!(
            name: "Seasoning",
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::Id(potato_ole_seasoning.id),
            default_item_ids: vec!(
                potato_ole_seasoning.id,
            )
        ),
        slot!(
            name: "Sauce",
            slot_type: SlotType::Items,
            selection: ItemSelection::AnyId(hashset!(nacho_cheese.id, guacamole.id, queso_fresco.id, sour_cream.id)),
            price_overrides: vec!(
                price_override!(tags: hashset!("sauces"), price: 99)
            )
        )
    ));

    let black_beans_and_rice = item!(
        id: ID::parse_str("076d422e-8b3e-42dc-8844-5c92caf6a9a1").unwrap(),
        long_name: "Black Beans and Rice",
        short_name: "SD-BBNR",
        price: 189, plu: "7413",
        tags: hashset!("side_of")
    );

    let side_of_shrimp = item!(
        id: ID::parse_str("0794119d-0762-403e-bb96-8ca5efdbbed3").unwrap(),
        long_name: "Shrimp",
        short_name: "SD-SHRIMP",
        price: 129, plu: "7943",
        tags: hashset!("side_of")
    );

    let side_of_cilantro_lime_rice = item!(
        id: ID::parse_str("07eb05af-bb77-48e1-b4ec-5346e651a1e0").unwrap(),
        long_name: "Cilantro Lime Rice",
        short_name: "SD-CLR",
        price: 189, plu: "7973",
        tags: hashset!("side_of")
    );

    let side_of_black_beans = item!(
        id: ID::parse_str("0acc1e85-1361-42a5-950e-458bacd479de").unwrap(),
        long_name: "Black Beans",
        short_name: "SD-BBN",
        price: 189, plu: "7983",
        tags: hashset!("side_of")
    );

    let side_of_refried_beans = item!(
        id: ID::parse_str("0b1cec50-0c07-4eea-8238-5c0468ee6744").unwrap(),
        long_name: "Refried Beans",
        short_name: "SD-BN",
        price: 265, plu: "8003",
        tags: hashset!("side_of")
    );

    let side_of_breakfast_bacon = item!(
        id: ID::parse_str("0b3741c2-7015-4a8b-b589-20efb8003ac0").unwrap(),
        long_name: "Bacon",
        short_name: "SD-BC",
        price: 110, plu: "8013",
        tags: hashset!("side_of")
    );

    let side_of_breakfast_sausage = item!(
        id: ID::parse_str("0bb44494-3a40-45f8-a33a-8cba00972296").unwrap(),
        long_name: "Sausage",
        short_name: "SD-SAU",
        price: 110, plu: "8023",
        tags: hashset!("side_of")
    );

    let side_of_cheddar_cheese = item!(
        id: ID::parse_str("0e1dab08-0198-418e-a66b-2bd9fb73a36e").unwrap(),
        long_name: "Cheddar Cheese",
        short_name: "SD-CH",
        price: 99, plu: "8033",
        tags: hashset!("side_of")
    );

    let side_of_chicken = item!(
        id: ID::parse_str("0ec822f5-407a-4e95-857f-227cadece6e1").unwrap(),
        long_name: "Chicken",
        short_name: "SD-CK",
        price: 149, plu: "8043",
        tags: hashset!("side_of")
    );

    let side_of_guacamole = item!(
        id: ID::parse_str("0fac58aa-3191-4de1-815e-d3b3836cbcac").unwrap(),
        long_name: "Guacamole",
        short_name: "SD-GUA",
        price: 129, plu: "8093",
        tags: hashset!("side_of")
    );

    let side_of_house_dressing = item!(
        id: ID::parse_str("0fb7cc17-fbf5-4675-9cb1-108689399b9a").unwrap(),
        long_name: "House Dressing",
        short_name: "SD-HD",
        price: 99, plu: "8113",
        tags: hashset!("side_of")
    );

    let side_of_jalapeno = item!(
        id: ID::parse_str("136933de-3624-44ab-b858-7f1c752030d5").unwrap(),
        long_name: "Jalapeno",
        short_name: "SD-JAL",
        price: 0, plu: "8123",
        tags: hashset!("side_of")
    );

    let side_of_lettuce = item!(
        id: ID::parse_str("1482b7c4-cd74-4dd8-b961-32121d32d390").unwrap(),
        long_name: "Lettuce",
        short_name: "SD-LET",
        price: 85, plu: "8133",
        tags: hashset!("side_of")
    );

    let side_of_beef = item!(
        id: ID::parse_str("153075ce-6254-4e94-bb51-d2b9f261bac5").unwrap(),
        long_name: "Beef",
        short_name: "SD-BF",
        price: 139, plu: "8143",
        tags: hashset!("side_of")
    );

    let side_of_mild_sauce = item!(
        id: ID::parse_str("15701ac3-a4c7-4e31-bc98-5cad8e1c3a17").unwrap(),
        long_name: "Mild Sauce",
        short_name: "SD-MS",
        price: 0, plu: "8153",
        tags: hashset!("side_of")
    );

    let side_of_nacho_cheese = item!(
        id: ID::parse_str("1662608f-6849-4afc-a542-cc69aaf3c776").unwrap(),
        long_name: "Nacho Cheese",
        short_name: "SD-NAC-CH",
        price: 99, plu: "8163",
        tags: hashset!("side_of", "quick")
    );

    let side_of_onions = item!(
        id: ID::parse_str("168c1c1d-8bbb-4cea-88bc-670e790c2bcf").unwrap(),
        long_name: "Onions",
        short_name: "SD-CRN-ON",
        price: 29, plu: "8193",
        tags: hashset!("side_of")
    );

    let side_of_pico_de_gallo = item!(
        id: ID::parse_str("169bc136-28fb-451e-9d59-4f2137c60285").unwrap(),
        long_name: "Pico De Gallo",
        short_name: "SD-PICO",
        price: 0, plu: "8203",
        tags: hashset!("side_of")
    );

    let side_of_ranch = item!(
        id: ID::parse_str("175ed97f-3251-4f6b-9183-f6c14ad22d28").unwrap(),
        long_name: "Ranch",
        short_name: "SD-RAN",
        price: 99, plu: "8213",
        tags: hashset!("side_of")
    );

    let side_of_queso_fresco = item!(
        id: ID::parse_str("190363ad-3dbd-4e74-af1b-a4a4b355b4ec").unwrap(),
        long_name: "Queso Fresco",
        short_name: "SD-QFR",
        price: 99, plu: "8223",
        tags: hashset!("side_of")
    );

    let side_of_salsa = item!(
        id: ID::parse_str("1967cdc9-4ee2-4f58-96be-66b55e08cc1f").unwrap(),
        long_name: "Salsa",
        short_name: "SD-SAL",
        price: 0, plu: "8233",
        tags: hashset!("side_of")
    );

    let side_of_potato_ole_seasoning = item!(
        id: ID::parse_str("1a10c946-5848-418c-a581-6a63891d9176").unwrap(),
        long_name: "Ole Seasoning",
        short_name: "SD-SEA",
        price: 99, plu: "8243",
        tags: hashset!("side_of")
    );

    let side_of_sour_cream = item!(
        id: ID::parse_str("1a5561cc-58a9-4790-a3cf-a8a35e13a156").unwrap(),
        long_name: "Sour Cream",
        short_name: "SD-SC",
        price: 99, plu: "8253",
        tags: hashset!("side_of", "quick")
    );

    let side_of_super_hot = item!(
        id: ID::parse_str("1b44e6f1-0e2c-492e-aaa0-35fbbcadd9bc").unwrap(),
        long_name: "Super Hot",
        short_name: "SD-SH",
        price: 0, plu: "8273",
        tags: hashset!("side_of")
    );

    let side_of_tomato = item!(
        id: ID::parse_str("1c8d7917-ddf0-4ee1-b7ae-883d82719b24").unwrap(),
        long_name: "Tomato",
        short_name: "SD-TOM",
        price: 50, plu: "8283",
        tags: hashset!("side_of")
    );

    let side_of_poblano_peppers = item!(
        id: ID::parse_str("1d11c955-e4c0-424a-bc1e-d72a8b64778d").unwrap(),
        long_name: "Poblano Peppers",
        short_name: "SD-PBLANO",
        price: 99, plu: "6543",
        tags: hashset!("side_of")
    );

    let side_of_fat_free_ranch = item!(
        id: ID::parse_str("1d37ce75-ceff-447c-8f71-1e213189e0d3").unwrap(),
        long_name: "Fat Free Ranch",
        short_name: "SD-FFRAN",
        price: 0, plu: "8413",
        tags: hashset!("side_of")
    );

    let side_of_thousand_island = item!(
        id: ID::parse_str("1e10ed60-42ff-43d4-af05-604978db1a76").unwrap(),
        long_name: "Thousand Island",
        short_name: "SD-TI",
        price: 0, plu: "8416",
        tags: hashset!("side_of")
    );

    let side_of_italian = item!(
        id: ID::parse_str("1e654159-5e05-4b02-a7d2-688a0bf11226").unwrap(),
        long_name: "Italian",
        short_name: "SD-ITAL",
        price: 0, plu: "8445",
        tags: hashset!("side_of")
    );

    let side_of_fat_free_french = item!(
        id: ID::parse_str("1ed6510f-1bd2-42a1-9754-7ace75f303c2").unwrap(),
        long_name: "Fat Free French",
        short_name: "SD-FFFREN",
        price: 0, plu: "8421",
        tags: hashset!("side_of")
    );

    let side_of_chimichurri = item!(
        id: ID::parse_str("1f9fba0d-8989-483f-bfcb-e25e64d3c5e5").unwrap(),
        long_name: "Chimichurri",
        short_name: "SD-CSC",
        price: 79, plu: "8673",
        tags: hashset!("side_of")
    );

    let side_of_pepperjack = item!(
        id: ID::parse_str("1fd44683-692b-460b-aed5-2f3ae0dbd29d").unwrap(),
        long_name: "Pepper Jack",
        short_name: "SD-PPR-JK",
        price: 79, plu: "8683",
        tags: hashset!("side_of")
    );

    let side_of_creamy_chipotle = item!(
        id: ID::parse_str("20cc6d66-a1fd-4dbd-a8c6-581250344cdc").unwrap(),
        long_name: "Creamy Chipotle",
        short_name: "SD-CCS",
        price: 79, plu: "8693",
        tags: hashset!("side_of")
    );

    let side_of_ketchup = item!(
        id: ID::parse_str("20e666fa-7580-4b11-8d61-50a233f15d73").unwrap(),
        long_name: "Ketchup",
        short_name: "SD-KETCH",
        price: 0, plu: "8720",
        tags: hashset!("side_of")
    );

    let side_of_lime_sour_cream = item!(
        id: ID::parse_str("2128b647-35e6-4080-9c94-4bc16a4d8eb8").unwrap(),
        long_name: "Lime Sour Cream",
        short_name: "SD-SC-LIM",
        price: 99, plu: "6553",
        tags: hashset!("side_of")
    );

    let side_of_fried_chicken = item!(
        id: ID::parse_str("216379be-e32d-46cb-a567-69e3935ae187").unwrap(),
        long_name: "Fried Chicken",
        short_name: "SD-FC",
        price: 589, plu: "12550",
        tags: hashset!("side_of"),
        slots: vec!(
            slot!(
                name: "Sauces",
                slot_type: SlotType::Replace,
                selection: ItemSelection::AnyId(hashset!(jalapeno_ranch.id, chipotle_lime.id, ranch.id))
            )
        )
    );

    let cheesy_bacon_nachos = item!(
        id: ID::parse_str("25f65a0f-e351-4c68-be1e-573b9e425c81").unwrap(),
        long_name: "Cheesy Bacon Nachos",
        short_name: "CH-BC-NACH",
        price: 200, plu: "12260",
        slots: vec!(slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                bacon.id,
                jalapeno.id,
                nacho_cheese.id,
                corn_chips.id,
                side_ranch.id
            )),
            default_item_ids: vec!(
                bacon.id,
                jalapeno.id,
                nacho_cheese.id,
                corn_chips.id,
                side_ranch.id
            )
        ),
    ));

    let cheesy_bacon_ranch_loaded_potatos = item!(
        id: ID::parse_str("26b97fb6-0f51-40a4-85cc-9ce6707092e4").unwrap(),
        long_name: "Cheesy Bacon Ranch Loaded Potatoes",
        short_name: "CH-BC-LOP",
        price: 300, plu: "12270",
        tags: hashset!("value", "secondaryside"),
        slots: vec!(slot!(
            name: "Ingredients",
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                bacon.id,
                nacho_cheese.id,
                potato_ole.id,
                side_ranch.id,
                potato_ole_seasoning.id
            )),
            default_item_ids: vec!(
                bacon.id,
                nacho_cheese.id,
                potato_ole.id,
                side_ranch.id,
                potato_ole_seasoning.id
            )
        ),
    ));

    let dressings = slot!(
        name: "Dressing",
        collapsed: false,
        minimum_quantity: 0,
        free_quantity: 1,
        slot_type: SlotType::Items,
        selection: ItemSelection::Tag("dressing".to_owned())
    );

    let add_ons = slot!( name: "Add Ons", slot_type: SlotType::Ingredient, collapsed: false, minimum_quantity: 0, selection: ItemSelection::AnyId(hashset!( jalapeno.id, onions.id, guacamole.id, refried_beans.id, pico_de_gallo.id, house_salsa.id, lettuce.id, tomato.id, sour_cream.id, poblano_peppers.id, bacon.id, potato_ole.id)));
    let add_ons_vec = vec![
        sour_cream.id,
        guacamole.id,
        super_hot.id,
        jalapeno.id,
        onions.id,
        refried_beans.id,
        pico_de_gallo.id,
        house_salsa.id,
        lettuce.id,
        tomato.id,
        poblano_peppers.id,
        bacon.id,
        ranch.id,
        ground_beef.id,
        chicken.id,
        fried_chicken.id,
        sausage.id,
        steak.id,
        potato_ole.id
    ];


    let side_salad = item!(
        id: ID::parse_str("2a58c07b-e1ca-49e3-be13-f3d650bcc816").unwrap(),
        long_name: "Side Salad",
        tags: hashset!("sides", "combosides"),
        short_name: "SAL",
        price: 265, plu: "3530",
        slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                lettuce.id,
                tomato.id,
                cheddar_cheese.id
            )),
            default_item_ids: vec!(
                lettuce.id,
                tomato.id,
                cheddar_cheese.id
            )
        ),
        dressings.clone(),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                lettuce.id,
                tomato.id
            )
            ,add_ons_vec.clone()
        ),
    ));

    let chips_and_nacho_cheese = item!(
        id: ID::parse_str("2a753fab-e2a0-4b88-aa90-1453c751fb9b").unwrap(),
        long_name: "Nachos",
        short_name: "Nachos",
        price: 249, plu: "2290",
        tags: hashset!("sides", "combosides"),
        slots: vec!(
            slot!(
                name: "Sauce",
                slot_type: SlotType::Items,
                minimum_quantity: 1,
                free_quantity: 1,
                selection: ItemSelection::AnyId(hashset!(nacho_cheese.id, guacamole.id, queso_fresco.id, sour_cream.id))
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(corn_chips.id)),
                default_item_ids: vec!(corn_chips.id)
            ),
            dynamic_add_ons(vec!(), add_ons_vec.clone())
        )
    );


    let sides = slot!(
        name: "Sides",
        slot_type: SlotType::Replace,
        collapsed: false,
        selection: ItemSelection::Tag("combosides".to_owned()),
        hidden: ItemSelection::Tag("secondaryside".into()),
        price_overrides: vec!(
            price_override!(item_ids: hashset!(potato_ole_side.id), price: 0, variation: Some("Small".into())),
            price_override!(item_ids: hashset!(potato_ole_side.id), price: 89, variation: Some("Medium".into())),
            price_override!(item_ids: hashset!(potato_ole_side.id), price: 129, variation: Some("Large".into())),
            price_override!(item_ids: hashset!(chips_and_nacho_cheese.id), price: 0),
            price_override!(item_ids: hashset!(side_salad.id), price: 0),
            price_override!(item_ids: hashset!(refried_beans.id), price: 0)
        ),
        default_item_ids: vec!(
            potato_ole_side.id
        ),
        default_quantity: 1,
        minimum_quantity: 1
    );

    // Beverage Stuff
    let beverage_variations = vec![
        variation!( name: "Kid", prefix: Some(String::from("KD")), identifier: "64", price: Some(199)),
        variation!( name: "Small", prefix: Some(String::from("SM")), identifier: "SM", price: Some(219)),
        variation!( name: "Medium", prefix: Some(String::from("MD")), identifier: "MD", price: Some(265)),
        variation!( name: "Large", prefix: Some(String::from("LG")), identifier: "LG", price: Some(295)),
    ];
    let kids_beverage_variations = vec![
        variation!( name: "Kid", prefix: Some(String::from("KD")), identifier: "64", price: Some(199)),
    ];

    let tea_additions = slot!(
        name: "Tea Additions",
        slot_type: SlotType::Ingredient,
        selection: ItemSelection::AnyId(hashset!(
            lemon.id,
            sweetener.id
            ))
    );

    let ice = item!(id: ID::parse_str("86be1a53-6c51-4f14-bfe7-ad40dff1037b").unwrap(), long_name: "Ice", short_name: "Ice", price: 0, plu: "999029", modifiers: vec!(Modification::Extra, Modification::Light));
    let beverage_slot = slot!(name:"Modifiers", slot_type: SlotType::Ingredient, selection: ItemSelection::AnyId(hashset!(ice.id)));
    let cold_drink_slots = slot!(name: "Cold Drink Additions", slot_type: SlotType::Ingredient, selection: ItemSelection::AnyId(hashset!(ice.id)), default_item_ids: vec!(ice.id));

    let starbucks_flavors = vec![
        variation!(
            name: "Vanilla",
            prefix: Some(String::from("Vn")),
            identifier: "Vanilla"
        ),
        variation!(
            name: "Mocha",
            prefix: Some(String::from("Mc")),
            identifier: "Mocha"
        ),
        variation!(
            name: "Carmel",
            prefix: Some(String::from("Cr")),
            identifier: "Carmel"
        ),
    ];

    let mountain_dew = item!(id: ID::parse_str("8725fe80-24d3-4581-941f-56c4e7ab218d").unwrap(), long_name: "Mountain Dew", short_name: "DEW", price: 0, plu: "7040", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let pepsi = item!(id: ID::parse_str("8804632e-79ec-4673-8df2-554c6dc4286f").unwrap(), long_name: "Pepsi", short_name: "P", price: 0, plu: "7000", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(),item_priority:2, slots: vec!(cold_drink_slots.clone()));
    let diet_pepsi = item!(id: ID::parse_str("89ce819c-f619-40b6-84b1-be4b54a91ca1").unwrap(), long_name: "Diet Pepsi", short_name: "DTP", price: 0, plu: "7010", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let sierra_mist = item!(id: ID::parse_str("8a0ab3e8-b3c7-4ad1-9b1e-3f4c4d7a4120").unwrap(), long_name: "Sierra Mist", short_name: "MST", price: 0, plu: "7060", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(),item_priority:2, slots: vec!(cold_drink_slots.clone()));
    let dr_pepper = item!(id: ID::parse_str("8b987ee4-0b5f-4204-b486-9d85963ca86b").unwrap(), long_name: "Dr Pepper", short_name: "DDR", price: 0, plu: "7020", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(),item_priority:2, slots: vec!(cold_drink_slots.clone()));
    let diet_mountain_dew = item!(id: ID::parse_str("8bdd06db-5724-4be2-b15d-002f95b0cd46").unwrap(), long_name: "Diet Mountain Dew", short_name: "DT-DEW", price: 0, plu: "7050", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let diet_sierray_mist = item!(id: ID::parse_str("8c19134d-018d-44e8-8081-27cd3f32563b").unwrap(), long_name: "Diet Sierra Mist", short_name: "DT-MST", price: 0, plu: "7100", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let cherry_pepsi = item!(id: ID::parse_str("8eccd25f-1fdf-4c60-8a8c-58ef11236354").unwrap(), long_name: "Cherry Pepsi", short_name: "CHP", price: 0, plu: "7140", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let diet_dr_pepper = item!(id: ID::parse_str("8f67b322-ab71-4395-86d5-1439695d1afc").unwrap(), long_name: "Diet Dr Pepper", short_name: "DT-DDR", price: 0, plu: "7150", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), item_priority:2,slots: vec!(cold_drink_slots.clone()));
    let mug_root_beer = item!(id: ID::parse_str("938c6a6d-cd4f-49b6-952c-c99144aa37cb").unwrap(), long_name: "Mug Root Beer", short_name: "Root Beer", price: 0, plu: "7030", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(),item_priority:2, slots: vec!(cold_drink_slots.clone()));

    let _coffee = item!(id: ID::parse_str("93e3e108-66ae-4cfe-bd5a-125a158271f5").unwrap(), long_name: "Coffee", short_name: "COFFEE", price: 159, plu: "7200", tags: hashset!("drinks", "includeddrink"),
    slots:
        vec!(
            beverage_slot.clone(),
            slot!(
                name: "Additions",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    cream.id,
                    sugar.id,
                    sweetener.id
                ))
            )
        ));

    let _decaf_coffee = item!(id: ID::parse_str("93ece057-69c0-4945-9162-45a821cb8786").unwrap(), long_name: "Decaf Coffee", short_name: "COFFEE-DECAF", price: 99, plu: "6095", tags: hashset!("drinks", "premiumdrink", "includeddrink"),
    slots:
        vec!(
            beverage_slot.clone(),
            slot!(
                name: "Additions",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    cream.id,
                    sugar.id,
                    sweetener.id
                ))
            )
        ));

    // Bottles
    let bottled_mtn_dew_code_red = item!(id: ID::parse_str("95429984-e0fa-4025-9e1e-da214c1027d4").unwrap(), long_name: "Mountain Dew code Red", short_name: "CODE-RED", price: 199, plu: "10540", item_priority:1, tags: hashset!("drinks", "premiumdrink"));
    let starbucks_frappachino = item!(id: ID::parse_str("954909de-4824-4a5c-9f78-4e01dd72214b").unwrap(), long_name: "Frappacino", short_name: "FRAP", price: 100, plu: "10550",tags: hashset!("drinks" ,"premiumdrink"),  item_priority:1, variations: starbucks_flavors.clone());
    let starbucks_double_shot = item!(id: ID::parse_str("956c11bc-e9b4-4051-a15d-0d0c2b65dd0f").unwrap(), long_name: "Double Shot", short_name: "DS", price: 100, plu: "10700",tags: hashset!("drinks" ,"premiumdrink"), item_priority:1,  variations: starbucks_flavors.clone());
    let starbucks_triple_shot = item!(id: ID::parse_str("95749489-d214-4472-9b07-a833827d19c5").unwrap(), long_name: "Triple Shot", short_name: "TS", price: 100, plu: "10750",tags: hashset!("drinks" ,"premiumdrink"),  item_priority:1, variations: starbucks_flavors.clone());

    let orange_juice = item!(id: ID::parse_str("9a12bb72-ff50-4b58-94b8-1565dbf093a9").unwrap(), long_name: "Orange Juice", short_name: "OJ", price: 279, plu: "5820",  item_priority:1, tags: hashset!("drinks", "premiumdrink"));
    let bottle_water = item!(id: ID::parse_str("9a346931-d498-4baa-85a3-b794ec91f622").unwrap(), long_name: "Bottle Water", short_name: "BOTTLE", price: 149, plu: "5850",  item_priority:1, tags: hashset!("drinks", "includeddrink"));
    let pomegranate_sobe = item!(id: ID::parse_str("9ae57bed-ff6a-46df-ab24-cd56cc5bf659").unwrap(), long_name: "Pomegranate Sobe Yumberry", short_name: "SOBE", price: 219, plu: "6030", item_priority:1,  tags: hashset!("drinks", "softdrink", "premiumdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone()));
    let lemonade = item!(id: ID::parse_str("9bcb7d5a-e29b-4049-9d77-1be3b60c4c3f").unwrap(), long_name: "Lemonade", short_name: "LEM", price: 0, plu: "7070", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone()));
    let water_cup = item!(id: ID::parse_str("9e7e54d1-0151-4aee-ab76-bdfc4c05d77f").unwrap(), long_name: "Water Cup", short_name: "WAT", price: 0, plu: "5000", tags: hashset!("drinks", "premiumdrink"));
    let iced_tea = item!(id: ID::parse_str("9f576bb7-ae87-47eb-bf78-ca64e3e95a87").unwrap(), long_name: "Iced Tea", short_name: "TEA", price: 0, plu: "7080", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone(), tea_additions.clone()));
    let peach_tea = item!(id: ID::parse_str("9f820416-5ab8-4ca9-a056-c8658c3c6d58").unwrap(), long_name: "Peach Tea", short_name: "TEA-P", price: 0, plu: "7180", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone(), tea_additions.clone()));
    let sweet_tea = item!(id: ID::parse_str("a07a9770-7b57-484c-bdb0-4fd28b313fab").unwrap(), long_name: "Sweet Tea", short_name: "STE", price: 0, plu: "7130", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone(), tea_additions.clone()));
    let fruit_punch = item!(id: ID::parse_str("a2a59244-5ce9-4c9a-b9f8-a6565403716a").unwrap(), long_name: "Fruit Punch", short_name: "PUN", price: 0, plu: "7160", tags: hashset!("drinks", "softdrink", "premiumdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone()));
    let raspberry_iced_tea = item!(id: ID::parse_str("a4f732c0-2c63-43af-9160-58ee226d798c").unwrap(), long_name: "Rasp. Iced Tea", short_name: "RSP", price: 0, plu: "7090", tags: hashset!("drinks", "softdrink"), variations: beverage_variations.clone(), slots: vec!(cold_drink_slots.clone(), tea_additions.clone()));
    let milk = item!(id: ID::parse_str("a513aabd-d8b4-401b-bd4b-537d195cb05a").unwrap(), long_name: "Milk", short_name: "MILK", price: 219, plu: "5800", tags: hashset!("drinks", "premiumdrink"));
    let chocolate_milk = item!(id: ID::parse_str("a656b3c2-3d7f-4669-b373-dd6c7e7835cc").unwrap(), long_name: "Chocolate Milk", short_name: "C-MILK", price: 215, plu: "7120", tags: hashset!("drinks", "premiumdrink"));

    let cold_brew_slot = slot!(
        name: "Ingredients",
        slot_type: SlotType::Ingredient,
        selection: ItemSelection::AnyId(hashset!(cane_syrup.id, mocha_pump.id, vanilla_pump.id))
    );
    let cold_brew_20oz = item!(
        id: ID::parse_str("33620d5b-fbbd-4d41-8c42-36fd570b6a01").unwrap(),
        long_name: "Cold Brew 20oz",
        short_name: "SM CBC",
        price: 200, plu: "7270",
        tags: hashset!("drinks", "premiumdrink", "includeddrink"),
        slots: vec!(
            cold_brew_slot.clone(),
            beverage_slot.clone()
        )
    );
    let cold_brew_plain = item!(
        id: ID::parse_str("450f8ce0-e099-44d0-ad4d-f88bf5aa193a").unwrap(),
        long_name: "Cold Brew Plain",
        short_name: "CBC-S",
        price: 200, plu: "7280",
        tags: hashset!("drinks", "premiumdrink", "includeddrink"),
        slots: vec!(
            cold_brew_slot.clone(),
            beverage_slot.clone()
        )
    );
    let cold_brew_vanilla = item!(
        id: ID::parse_str("59b2c807-5f04-4175-aa00-62f6eec040eb").unwrap(),
        long_name: "Cold Brew Vanilla",
        short_name: "CBC-V",
        price: 200, plu: "7290",
        tags: hashset!("drinks", "premiumdrink", "includeddrink"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(vanilla_pump.id, cane_syrup.id, mocha_pump.id))
            ),
            beverage_slot.clone()
        )
    );
    let cold_brew_mocha = item!(
        id: ID::parse_str("13825a91-b0ce-47ad-bdd3-3db7dd8279e2").unwrap(),
        long_name: "Cold Brew Mocha",
        short_name: "CBC-M",
        price: 200, plu: "7300",
        tags: hashset!("drinks", "premiumdrink", "includeddrink"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(vanilla_pump.id, cane_syrup.id, mocha_pump.id))
            ),
            beverage_slot.clone()
        )
    );
    let cold_brew_unsweet = item!(
        id: ID::parse_str("414d5794-b9fd-4ba7-8ff6-f6c9f79b5f98").unwrap(),
        long_name: "Cold Brew Unsweet",
        short_name: "CBC-X",
        price: 200, plu: "7310",
        tags: hashset!("drinks", "premiumdrink", "includeddrink"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(vanilla_pump.id, cane_syrup.id, mocha_pump.id))
            ),
            beverage_slot.clone()
        )
    );

    let kids_beverages = slot!(
        name: "Drinks",
        slot_type: SlotType::Replace,
        price_overrides: vec!(
        price_override!(tags: hashset!("softdrink"), variation: Some("Kid".into())),
        ),
        selection: ItemSelection::AnyTag(hashset!("drinks")),
        default_quantity: 1,
        minimum_quantity: 1
    );

    let drinks = slot!(
        name: "Drinks",
        slot_type: SlotType::Replace,
        price_overrides: vec!(
        price_override!(tags: hashset!("includeddrink"), price: 0),
        price_override!(item_ids: hashset!(orange_juice.id), price: 60),
        price_override!(tags: hashset!("softdrink"), variation: Some("Small".into()), price: 0),
        price_override!(tags: hashset!("softdrink"), variation: Some("Medium".into()), price: 80),
        price_override!(tags: hashset!("softdrink"), variation: Some("Large".into()), price: 110),
        price_override!(tags: hashset!("premiumdrink"), price: 110),
        ),
        selection: ItemSelection::AnyTag(hashset!("drinks")),
        hidden: ItemSelection::AnyTag(hashset!("premiumdrink")),
        default_quantity: 1,
        minimum_quantity: 1
    );

    fn dynamic_add_ons(ingredients: Vec<ID>, add_ons: Vec<ID>) -> SlotDefinition {
        let final_add_ons: BTreeSet<ID> = add_ons
            .iter()
            .filter(|item| !ingredients.contains(item))
            .cloned()
            .collect();
        slot!( name: "Add Ons", slot_type: SlotType::Ingredient, collapsed: false, minimum_quantity: 0, selection: ItemSelection::AnyId(final_add_ons))
    }

    let dessert_add_ons = slot!(
        name: "Add Ons",
        slot_type: SlotType::Ingredient,
        collapsed: false,
        minimum_quantity: 0,
        selection: ItemSelection::AnyId(hashset!(cream_cheese_icing.id, cinnamon_sugar.id)),
        price_overrides: vec!(
            price_override!(tags: hashset!("addon"), price: 25, item_ids: hashset!(cream_cheese_icing.id, cinnamon_sugar.id))
    ));

    let snack_chicken_quesadilla = item!(
        id: ID::parse_str("26e2dc82-1b09-4483-b63f-62a480a9000c").unwrap(),
        long_name: "Snack Chicken Quesadilla",
        short_name: "SNK-QUES-CK",
        price: 100, plu: "12300",
        tags: hashset!("value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(cheddar_cheese.id, small_tortilla.id, chicken.id)),
                default_item_ids: vec!(cheddar_cheese.id, small_tortilla.id, chicken.id)
            ),
            dynamic_add_ons(
                vec!(cheddar_cheese.id)
                , add_ons_vec.clone()
            )
        )
    );

    let chicken_bacon_guac_street_taco = item!(
        id: ID::parse_str("c36a55db-e2bb-4ddb-95d6-f2c7f462e5b8").unwrap(),
        long_name: "Chkn Bcn Guac Taco",
        short_name: "ST-CBG",
        price: 200, plu: "12280",
        tags: hashset!("quick", "value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(chicken.id, bacon.id, guacamole.id, cheddar_cheese.id, lettuce.id, small_tortilla.id)),
                default_item_ids: vec!(chicken.id, bacon.id, guacamole.id, cheddar_cheese.id, lettuce.id, small_tortilla.id)
            ),
            dynamic_add_ons(
                vec!(chicken.id, bacon.id, guacamole.id, cheddar_cheese.id, lettuce.id, small_tortilla.id)
                , add_ons_vec.clone()
            )
        )
    );

    let fried_chicken_potato_snacker = item!(
        id: ID::parse_str("27fe823f-1d98-43bd-bf1a-8606c45c5d78").unwrap(),
        long_name: "Fried Chicken Potato Ole Snacker",
        short_name: "SNK-FC-PO",
        price: 449, plu: "12540",
        tags: hashset!("value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(potato_ole.id, fried_chicken.id)),
                default_item_ids: vec!(potato_ole.id, fried_chicken.id)
            ),
            slot!(
                name: "Sauce",
                collapsed: false,
                slot_type: SlotType::Replace,
                free_quantity: 1,
                selection: ItemSelection::AnyId(hashset!(
                    jalapeno_ranch.id,
                    chipotle_lime.id,
                    ranch.id
                )),
                minimum_quantity: 1,
                maximum_quantity: 1
            ),
            dynamic_add_ons(
                vec!(potato_ole.id, fried_chicken.id, ranch.id)
                , add_ons_vec.clone()
            )
        )
    );

    let snack_cheese_quesadilla = item!(
        id: ID::parse_str("290da8ba-7956-49c4-8b4d-8ded9d12e442").unwrap(),
        long_name: "Snack Cheese Quesadilla",
        short_name: "SNK-QUES-CH",
        price: 199, plu: "1960",
        tags: hashset!("sides"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(cheddar_cheese.id, small_tortilla.id)),
                default_item_ids: vec!(cheddar_cheese.id, small_tortilla.id)
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    small_tortilla.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let _cheesy_rollup = item!(
        id: ID::parse_str("2ec0ece1-b573-4e42-8174-af927af398ae").unwrap(),
        long_name: "Cheese Rollup",
        short_name: "ROLL-CHZ",
        price: 199, plu: "3910",
        tags: hashset!("sides"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    small_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    small_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    small_tortilla.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let _cinnamon_sugar_tortilla_crisps = item!(
        id: ID::parse_str("307902b3-6853-49e0-9034-478fda7d543b").unwrap(),
        long_name: "Cinnamon Sugar Tortilla Crisps",
        short_name: "CIN-CHIPS",
        price: 100, plu: "12290",
        tags: hashset!("dessert", "value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    cinnamon_sugar.id
                )),
                default_item_ids: vec!(
                    cinnamon_sugar.id
                )
            )
        )
    );

    let _mexican_donut_bites = item!(
        id: ID::parse_str("30acc589-5d81-4f30-8e9a-55e69542c144").unwrap(),
        long_name: "Mexican Donut Bites",
        short_name: "D-BITES",
        price: 239, plu: "2860",
        tags: hashset!("dessert"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    cream_cheese_icing.id,
                    cinnamon_sugar.id
                )),
                default_item_ids: vec!(
                    cream_cheese_icing.id,
                    cinnamon_sugar.id
                )
            ),
            dessert_add_ons.clone()
        )
    );

    let quesadilla = item!(
        id: ID::parse_str("33e07d2a-7b5d-4b92-8437-f2d25a46072c").unwrap(),
        long_name: "Cheese Quesadilla",
        short_name: "QUES-CH",
        price: 399, plu: "2500",
        tags: hashset!("dinner", "lunch", "specialties"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(ground_beef.id, chicken.id, fried_chicken.id, cheese.id, steak.id)),
                default_item_ids: vec!(cheese.id)
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    nacho_cheese.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    nacho_cheese.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    large_tortilla.id
                ),
                add_ons_vec.clone()
            )
        )
    );

    let steak_quesadilla = item!(
        id: ID::parse_str("3572b3f2-1b26-42d4-a690-072c66ce43f0").unwrap(),
        long_name: "Steak Quesadilla",
        short_name: "QUES-STK",
        price: 589, plu: "2520",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    large_tortilla.id
                ),
                add_ons_vec.clone()
            )
        )
    );

    let beef_quesadilla = item!(
        id: ID::parse_str("365940ac-950e-451b-ba09-0f3ef5640c36").unwrap(),
        long_name: "Beef Quesadilla",
        short_name: "QUES-BF",
        price: 499, plu: "2501",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    large_tortilla.id
                ),
                add_ons_vec.clone()
            )
        )
    );

    let chicken_quesadilla = item!(
        id: ID::parse_str("37104689-b96a-4777-a9f1-8396f615e087").unwrap(),
        long_name: "Chicken Quesadilla",
        short_name: "QUES-CK",
        price: 545, plu: "2510",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    large_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    large_tortilla.id
                ),
                add_ons_vec.clone()
            )
        )
    );

    let _cheesy_quesadilla = item!(
        id: ID::parse_str("3739089c-5838-419a-80a6-9e8b5652c6ef").unwrap(),
        long_name: "Cheesy Quesadilla",
        short_name: "Chsy Ques",
        price: 259, plu: "9220",
        tags: hashset!("dinner", "lunch"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    large_tortilla.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    large_tortilla.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    large_tortilla.id
                ),
                add_ons_vec.clone()
            )
        )
    );

    let beef_super_potato_ole = item!(
        id: ID::parse_str("375fd04f-b156-4f79-a5b5-1153764e817f").unwrap(),
        long_name: "Super Potato Oles",
        short_name: "SPO",
        price: 719, plu: "2200",
        slots: vec!(
            taco_proteins.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_super_potato_ole = item!(
        id: ID::parse_str("4f10fe83-a1d9-4d05-b936-5174d4f1f9a2").unwrap(),
        long_name: "Chicken Super Potato Oles",
        short_name: "SPO CHK",
        price: 729, plu: "2210",
        slots: vec!(
            taco_proteins.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_super_potato_ole = item!(
        id: ID::parse_str("754f285e-89e7-4a8e-9c45-94b752b9d533").unwrap(),
        long_name: "Steak Super Potato Oles",
        short_name: "SPO STK",
        price: 789, plu: "2220",
        slots: vec!(
            taco_proteins.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_super_potato_ole = item!(
        id: ID::parse_str("972d7a92-4d43-41a1-9726-80fde50ef960").unwrap(),
        long_name: "Bean Super Potato Oles",
        short_name: "SPO BN",
        price: 719, plu: "2240",
        slots: vec!(
            taco_proteins.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    tomato.id,
                    guacamole.id,
                    sour_cream.id,
                    potato_ole_seasoning.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let super_potato_ole_shell_item = item!(
        id: ID::parse_str("980d7c13-55ee-4ed1-9cd1-77dd857eb628").unwrap(),
        long_name: "Super Potato Ole",
        short_name: "SPOL",
        tags: hashset!("specialties", "quick"),
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Super Potato Ole",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_super_potato_ole.id, chicken_super_potato_ole.id, steak_super_potato_ole.id, bean_super_potato_ole.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_super_potato_ole.id)
            ),
        )
    );

    let _super_nachos = item!(
        id: ID::parse_str("3b01b549-d23d-44eb-9468-f28c31e76629").unwrap(),
        long_name: "Super Nachos",
        short_name: "SN",
        price: 719, plu: "2300",
        tags: hashset!("dinner", "lunch"),
        variations: super_size_variations.clone(),
        default_variation: super_size_variations[0].id,
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::Tag("proteins".to_owned()),
                default_item_ids: vec!(ground_beef.id, refried_beans.id)
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    cheddar_cheese.id,
                    nacho_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    cheddar_cheese.id,
                    nacho_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    nacho_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_super_nachos = item!(
        id: ID::parse_str("3c096c23-a8c5-4f77-bdae-a0a7ef8b2e6b").unwrap(),
        long_name: "Super Nachos",
        short_name: "SN",
        price: 719, plu: "2300",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_super_nachos = item!(
        id: ID::parse_str("3c4eb7e4-7961-4763-bf86-6f81d5662821").unwrap(),
        long_name: "Chicken Super Nachos",
        short_name: "NACH-SUP-CK",
        price: 765, plu: "2310",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    chicken.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_super_nachos = item!(
        id: ID::parse_str("3dab11ac-7f04-4dac-b19e-ca0fc09bb4f9").unwrap(),
        long_name: "Steak Super Nachos",
        short_name: "NACH-SUP-STK",
        price: 779, plu: "2320",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    steak.id,
                    cheddar_cheese.id,
                    corn_chips.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    pico_de_gallo.id,
                    refried_beans.id,
                    guacamole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_taco_salad = item!(
        id: ID::parse_str("3e2a4f92-ab59-428c-b4c9-a389aa2b360f").unwrap(),
        long_name: "Taco Salad",
        short_name: "TSAL-BF",
        price: 729, plu: "2600",
        slots: vec!(
            dressings.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
                ,add_ons_vec.clone()
            ),
        )
    );

    let chicken_taco_salad = item!(
        id: ID::parse_str("51ff7da4-ccc5-4c5b-85a5-0d3dedf99e2c").unwrap(),
        long_name: "Taco Salad CK",
        short_name: "TSAL-CK",
        price: 799, plu: "2610",
        slots: vec!(
            dressings.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
                ,add_ons_vec.clone()
            ),
        )
    );

    let steak_taco_salad = item!(
        id: ID::parse_str("9b4f3071-ad55-4c4d-9f68-c18fec3bcf93").unwrap(),
        long_name: "Taco Salad Stk",
        short_name: "TSAL-STK",
        price: 799, plu: "2620",
        slots: vec!(
            dressings.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
                ,add_ons_vec.clone()
            ),
        )
    );

    let bean_taco_salad = item!(
        id: ID::parse_str("bfe4fbd8-347f-4b0f-963d-544fb7772598").unwrap(),
        long_name: "Taco Salad Bn",
        short_name: "TSAL-BN",
        price: 729, plu: "2640",
        slots: vec!(
            dressings.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    house_salsa.id,
                    taco_bowl.id,
                    lettuce.id,
                    tomato.id,
                    cheddar_cheese.id,
                    sour_cream.id
                )
                ,add_ons_vec.clone()
            ),
        )
    );

    let taco_salad_shell_item = item!(
        id: ID::parse_str("98ac2b3e-7fb6-47cc-8577-5c52bec50fb8").unwrap(),
        long_name: "Taco Salad",
        short_name: "Taco",
        tags: hashset!("dinner", "lunch", "specialties"),
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Taco Salad",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_taco_salad.id, chicken_taco_salad.id, steak_taco_salad.id, bean_taco_salad.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_taco_salad.id)
            ),
        )
    );

    let _loaded_nachos = item!(
        id: ID::parse_str("3e3cbef1-8aa5-42bf-b8ac-3547ed656340").unwrap(),
        long_name: "Loaded Nachos",
        short_name: "LN",
        price: 0, plu: "6330",
        tags: hashset!("dinner", "lunch"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    corn_chips.id,
                    sour_cream.id,
                    poblano_peppers.id,
                    pico_de_gallo.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    corn_chips.id,
                    sour_cream.id,
                    poblano_peppers.id,
                    pico_de_gallo.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    steak.id,
                    corn_chips.id,
                    sour_cream.id,
                    poblano_peppers.id,
                    pico_de_gallo.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let _potato_ole_scrambler = item!(
        id: ID::parse_str("3e7a774b-3546-4e54-ae61-9a0db3348be9").unwrap(),
        long_name: "Potato Ole Scrambler",
        short_name: "POS",
        price: 729, plu: "4150",
        tags: hashset!("breakfast"),
        item_priority: 11,
        slots: vec!(
            slot!(
                name: "Protein",
                collapsed: false,
                slot_type: SlotType::Replace,
                selection: ItemSelection::AnyId(hashset!(
                    sausage.id,
                    bacon.id
                )),
                price_overrides: vec!(
                    price_override!(tags: hashset!("breakfast_protein"), price: 0, item_ids: hashset!( sausage.id, bacon.id))
            )
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    scrambled_eggs.id,
                    bacon.id,
                    sausage.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    scrambled_eggs.id,
                    bacon.id,
                    sausage.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    bacon.id,
                    sausage.id,
                    nacho_cheese.id,
                    cheddar_cheese.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id

                )
                ,add_ons_vec.clone()
            )
        )
    );

    let spicy_chorizo_breakfast_burrito = item!(
        id: ID::parse_str("3f94a710-e2ad-4f32-bd2c-2291d6830957").unwrap(),
        long_name: "Spicy Chorizo Breakfast Burrito",
        short_name: "BUR-SPCY",
        price: 395, plu: "4600",
        tags: hashset!("breakfast"),
        item_priority: 12,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    jalapeno.id,
                    super_hot.id
                )),
                default_item_ids: vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    jalapeno.id,
                    super_hot.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    jalapeno.id,
                    super_hot.id
                )
                ,add_ons_vec.clone()
            )
        )
    );
    
    let beef_potato_breakfast_burrito = item!(
        id: ID::parse_str("b0f1f629-1303-48e0-be50-033a7310eb77").unwrap(),
        long_name: "Beef M&P Breakfast Burrito",
        short_name: "BKMPB-BF",
        price: 389, plu: "4702",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    ground_beef.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id

                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bacon_potato_breakfast_burrito = item!(
        id: ID::parse_str("f7e4219c-9dec-448b-9a26-1d114406601e").unwrap(),
        long_name: "Bacon M&P Breakfast Burrito",
        short_name: "BKMPB-BA",
        price: 389, plu: "4700",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    bacon.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )),
                default_item_ids: vec!(
                    bacon.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id

                )
                ,add_ons_vec.clone()
            )
        )
    );

    let sausage_potato_breakfast_burrito = item!(
        id: ID::parse_str("ef95c69e-2d38-4a71-8e52-994d33d36cc3").unwrap(),
        long_name: "Sausage M&P Breakfast Burrito",
        short_name: "BKMPB-SA",
        price: 389, plu: "4710",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    sausage.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )),
                default_item_ids: vec!(
                    sausage.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id

                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_potato_breakfast_burrito = item!(
        id: ID::parse_str("4163382d-3eed-46cb-aaa7-f8a02b8b8f0b").unwrap(),
        long_name: "Steak M&P Breakfast Burrito",
        short_name: "BKMPB-STK",
        price: 499, plu: "3060",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id

                )
                ,add_ons_vec.clone()
            )
        )
    );

    let spicy_chicken_and_potato_griller = item!(
        id: ID::parse_str("416514e0-dc30-443f-a7da-cf4548bba3c0").unwrap(),
        long_name: "Spicy Chicken and Potato Griller",
        short_name: "ORGRL-CK",
        price: 200, plu: "12310",
        item_priority: 12,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    chicken.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_and_potato_griller = item!(
        id: ID::parse_str("be2d02d0-2971-4780-8614-0ac7060b6357").unwrap(),
        long_name: "Steak and Potato Griller",
        short_name: "Spicy S&P Griller",
        price: 300, plu: "12320",
        item_priority: 12,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    steak.id,
                    creamy_chipotle.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    super_hot.id,
                    med_tortilla.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let potato_griller_shell_item = item!(
        id: ID::parse_str("3d706676-19f8-43af-8aef-8065b84eedf0").unwrap(),
        long_name: "Meat and Potato Griller",
        short_name: "M&P Griler",
        tags: hashset!("value"),
        item_priority: 20,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(spicy_chicken_and_potato_griller.id, steak_and_potato_griller.id)),
                minimum_quantity: 1
            ),
        )
    );

    let triple_meat_potato_breakfast_burrito = item!(
        id: ID::parse_str("42626cf1-f04f-44d2-b0bb-99e172e802fa").unwrap(),
        long_name: "Triple Meat and Potato Breakfast Burrito",
        short_name: "TRIP-MT-B",
        price: 449, plu: "6430",
        item_priority: 13,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    bacon.id,
                    sausage.id,
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    tomato.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )),
                default_item_ids: vec!(
                    bacon.id,
                    sausage.id,
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    tomato.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    bacon.id,
                    sausage.id,
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    tomato.id,
                    potato_ole.id,
                    potato_ole_seasoning.id,
                    large_tortilla.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bacon_scrambler_burrito = item!(
        id: ID::parse_str("429300cc-b31d-4f11-ab8b-50d76dc02b6b").unwrap(),
        long_name: "Bacon Scrambler Burrito",
        short_name: "SCRAM-BUR-BA",
        price: 419, plu: "4100",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    bacon.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    bacon.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let sausage_scrambler_burrito = item!(
        id: ID::parse_str("429eaa27-c3c4-44c6-982d-2a532d8f43f3").unwrap(),
        long_name: "Sausage Scrambler Burrito",
        short_name: "SCRAM-BUR-SAU",
        price: 419, plu: "4110",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    sausage.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    sausage.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_scrambler_burrito = item!(
        id: ID::parse_str("42d0c56b-5d8c-499d-b72e-74acb45e48cf").unwrap(),
        long_name: "Steak Scrambler Burrito",
        short_name: "SCRAM-BUR-STK",
        price: 489, plu: "3070",
        slots: vec!(
            breakfast_proteins.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    scrambled_eggs.id,
                    nacho_cheese.id,
                    large_tortilla.id,
                    potato_ole.id,
                    poblano_peppers.id,
                    onions.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let scrambler_burrito_shell_item = item!(
        id: ID::parse_str("539790d3-ec2d-4f91-bac9-f1937e85cb82").unwrap(),
        long_name: "Scrambler Burrito",
        short_name: "Scr Burr",
        tags: hashset!("breakfast"),
        item_priority: 85,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(bacon_scrambler_burrito.id, sausage_scrambler_burrito.id, steak_scrambler_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(bacon_scrambler_burrito.id)
            ),
        )
    );

    let bacon_junior_breakfast_burrito = item!(
        id: ID::parse_str("43922e21-fcb8-4d56-b331-4f1b1c987227").unwrap(),
        long_name: "Bacon Jr. Breakfast Burrito",
        short_name: "BUR-JBA",
        price: 239, plu: "3650",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    bacon.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )),
                default_item_ids: vec!(
                    bacon.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let sausage_junior_breakfast_burrito = item!(
        id: ID::parse_str("45e44a2c-01b2-4f76-b140-c311f8a0c5c6").unwrap(),
        long_name: "Sausage Jr. Breakfast Burrito",
        short_name: "BUR-JSAU",
        price: 239, plu: "3660",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    sausage.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )),
                default_item_ids: vec!(
                    sausage.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_junior_breakfast_burrito = item!(
        id: ID::parse_str("46503465-9dce-43b2-ac03-9281af1c91fb").unwrap(),
        long_name: "Steak Jr. Breakfast Burrito",
        short_name: "BUR-JSTK",
        price: 339, plu: "3140",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    nacho_cheese.id,
                    mild_sauce.id,
                    small_tortilla.id,
                    scrambled_eggs.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let junior_breakfast_burrito_shell_item = item!(
        id: ID::parse_str("faa1ef25-78d4-4b4b-ba1a-e59e285fe408").unwrap(),
        long_name: "Junior Breakfast Burrito",
        short_name: "J Brk Burr",
        tags: hashset!("breakfast"),
        item_priority: 85,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(bacon_junior_breakfast_burrito.id, sausage_junior_breakfast_burrito.id, steak_junior_breakfast_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(bacon_junior_breakfast_burrito.id)
            ),
        )
    );

    let boss_sauce = slot!(
        name: "Sauce",
        slot_type: SlotType::Items,
        minimum_quantity: 1,
        maximum_quantity: 4,
        selection: ItemSelection::AnyId(hashset!(pico_de_gallo.id, corn_salsa.id)),
        price_overrides: vec!(
            price_override!(item_ids: hashset!(pico_de_gallo.id, corn_salsa.id), price: 0)
        )
    );

    let beef_boss_bowl = item!(
        id: ID::parse_str("48d3360b-f6ba-42a8-a5fe-91766db9af98").unwrap(),
        long_name: "Beef Boss Bowl",
        short_name: "B-BWL-BF",
        price: 629, plu: "12030",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_boss_bowl = item!(
        id: ID::parse_str("48fb0de9-6f28-4209-b58b-277637e30718").unwrap(),
        long_name: "Chicken Boss Bowl",
        short_name: "B-BWL-CK",
        price: 699, plu: "12040",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_boss_bowl = item!(
        id: ID::parse_str("49f4d3a7-5ef3-4888-9c36-5a31655e09b3").unwrap(),
        long_name: "Steak Boss Bowl",
        short_name: "B-BWL-STK",
        price: 799, plu: "12050",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_boss_burrito = item!(
        id: ID::parse_str("4a5f9ef1-50a6-4ea5-b62e-0c6e8d9fd2a8").unwrap(),
        long_name: "Steak Boss Burrito",
        short_name: "B-BUR-STK",
        price: 799, plu: "12020",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_boss_burrito = item!(
        id: ID::parse_str("4a75d7e9-60a5-4f06-9a3f-583904e1a0d5").unwrap(),
        long_name: "Beef Boss Burrito",
        short_name: "B-BUR-BF",
        price: 629, plu: "12000",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_boss_burrito = item!(
        id: ID::parse_str("4bd037df-bc9c-45f6-bfd3-0e91d3f05d6a").unwrap(),
        long_name: "Chicken Boss Burrito",
        short_name: "B-BUR-CK",
        price: 699, plu: "12010",
        slots: vec!(
            boss_sauce.clone(),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    cheddar_cheese.id,
                    sour_cream.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    house_salsa.id,
                    black_beans.id,
                    cilantro_lime_rice.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_meat_potato_burrito = item!(
        id: ID::parse_str("4d8ae5f3-abab-4975-87c5-596e722b94b6").unwrap(),
        long_name: "Steak M&P  Burrito",
        short_name: "MPB-STK",
        price: 549, plu: "520",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_meat_potato_burrito = item!(
        id: ID::parse_str("4fbd95a4-1ba9-40ac-9ec7-853ed45c8758").unwrap(),
        long_name: "Beef M&P Burrito",
        short_name: "MPB-BF",
        price: 399, plu: "500",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_meat_potato_burrito = item!(
        id: ID::parse_str("51814ea5-2c97-472e-8ebc-c903526051f2").unwrap(),
        long_name: "Chicken M&P Burrito",
        short_name: "MPB-CK",
        price: 499, plu: "510",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let fried_chicken_meat_potato_burrito = item!(
        id: ID::parse_str("2bab8913-49ee-4e6e-9e4e-e1486ec8733e").unwrap(),
        long_name: "FC M&P Burrito",
        short_name: "MPB-FC",
        price: 489, plu: "12520",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    fried_chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_meat_potato_burrito = item!(
        id: ID::parse_str("527f423c-50ef-4790-ba2d-a916a2725a3d").unwrap(),
        long_name: "Bean M&P Burrito",
        short_name: "MPB-BN",
        price: 399, plu: "540",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    fried_chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    fried_chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let saus_and_bacon_meat_potato_burrito = item!(
        id: ID::parse_str("c714c46c-e2a0-4480-82ee-8c83bd63c841").unwrap(),
        long_name: "Half BCN SSG M&P Burrito",
        short_name: "MPB-BACSAU",
        price: 389, plu: "4701",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    bacon.id,
                    sausage.id,
                    large_tortilla.id,
                    sour_cream.id,
                    lettuce.id,
                    nacho_cheese.id,
                    potato_ole.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    bacon.id,
                    sausage.id,
                    fried_chicken.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    bacon.id,
                    sausage.id,
                    large_tortilla.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    lettuce.id,
                    potato_ole.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );
    let half_and_half = item!(id: ID::parse_str("aec2f71c-5a4b-48ee-8458-0634d0f934f0").unwrap(), long_name: "Half Bacon Suasage", short_name: "BACSAU", price: 0, plu: "999124", tags: hashset!("breakfast_proteins"), modifiers: vec!(Modification::Extra, Modification::Light));

    let meat_and_potato_breakfast_burrito_shell_item = item!(
        id: ID::parse_str("3208567a-3716-4c32-ae17-5b6d33ba20f1").unwrap(),
        long_name: "M&P Breakfast Burrito",
        short_name: "Taco",
        tags: hashset!("breakfast", "quick"),
        item_priority: 91,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(bacon_potato_breakfast_burrito.id, beef_potato_breakfast_burrito.id, sausage_potato_breakfast_burrito.id, steak_potato_breakfast_burrito.id, saus_and_bacon_meat_potato_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(bacon_potato_breakfast_burrito.id)
            ),
        )
    );

    let combination_burrito = item!(
        id: ID::parse_str("52f47ccc-c137-458b-b2dc-c087c39fa2f0").unwrap(),
        long_name: "Combination Burrito",
        short_name: "BUR-CB",
        price: 319, plu: "490",
        tags: hashset!("dinner", "lunch"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::Tag("proteins".to_owned()),
                default_item_ids: vec!(ground_beef.id, refried_beans.id)
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_burrito = item!(
        id: ID::parse_str("53a21d3f-77d0-4a09-8eac-fe7ac535871c").unwrap(),
        long_name: "Bean Burrito",
        short_name: "BUR-BN",
        price: 219, plu: "400",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 1,
        slots: vec!(
            /*slot!(
                name: "Protein",
                collapsed: true,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id
                )),
                default_item_ids: vec!(
                    refried_beans.id
                )
            ),*/
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    refried_beans.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_burrito = item!(
        id: ID::parse_str("54642316-47b2-4215-9fe2-e81e7663ded7").unwrap(),
        long_name: "Steak Burrito",
        short_name: "BUR-STK",
        price: 540, plu: "420",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_burrito = item!(
        id: ID::parse_str("549c0676-5ebd-4695-9a83-5669c2c124f0").unwrap(),
        long_name: "Beef Burrito",
        short_name: "BUR-BF",
        price: 335, plu: "480",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_burrito = item!(
        id: ID::parse_str("54c610fa-a45a-4317-b960-252bb5e8b72a").unwrap(),
        long_name: "Chicken Burrito",
        short_name: "BUR-CK",
        price: 379, plu: "410",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let fried_chicken_burrito = item!(
        id: ID::parse_str("a258eacc-d7d0-44cc-8b43-579ccdd267f9").unwrap(),
        long_name: "Fried Chicken Burrito",
        short_name: "BUR-FC",
        price: 489, plu: "12520",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    fried_chicken.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )),
                default_item_ids: vec!(
                    fried_chicken.id,
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    large_tortilla.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_super_burrito = item!(
        id: ID::parse_str("5c8b8314-e3b6-4c8b-a0a4-df904223b576").unwrap(),
        long_name: "Steak Super Burrito",
        short_name: "SB-STK",
        price: 529, plu: "620",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    refried_beans.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    onions.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_super_burrito = item!(
        id: ID::parse_str("5cab3ebd-5877-4aff-83bd-e9b4652c367c").unwrap(),
        long_name: "Super Burrito",
        short_name: "SB",
        price: 399, plu: "600",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    refried_beans.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    onions.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_super_burrito = item!(
        id: ID::parse_str("5eb467bd-0f1f-4ad9-ae4f-d39fbe579bb4").unwrap(),
        long_name: "Chicken Super Burrito",
        short_name: "SB-CK",
        price: 479, plu: "610",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    refried_beans.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    onions.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_super_burrito = item!(
        id: ID::parse_str("5efdabd6-2570-490a-b841-09e7737ec5e4").unwrap(),
        long_name: "Bean Super Burrito",
        short_name: "SB-BN",
        price: 379, plu: "640",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    onions.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    cheddar_cheese.id,
                    lettuce.id,
                    large_tortilla.id,
                    tomato.id,
                    sour_cream.id,
                    mild_sauce.id,
                    onions.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_crispy_taco = item!(
        id: ID::parse_str("5ff7793a-a134-45f4-be87-672d0f8eb083").unwrap(),
        long_name: "Crispy Taco Chicken",
        short_name: "TACO-CK",
        price: 223, plu: "110",
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                chicken.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )),
            default_item_ids: vec!(
                chicken.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_crispy_taco = item!(
        id: ID::parse_str("61edad29-b481-4f68-9ce9-905fb13d80e6").unwrap(),
        long_name: "Crispy Taco Steak",
        short_name: "TACO-STK",
        price: 269, plu: "120",
        item_priority: 3,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                steak.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )),
            default_item_ids: vec!(
                steak.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_crispy_taco = item!(
        id: ID::parse_str("62e6963c-8cbe-4d11-9b90-92441fad50d6").unwrap(),
        long_name: "Crispy Taco",
        short_name: "TACO",
        price: 169, plu: "100",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                ground_beef.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )),
            default_item_ids: vec!(
                ground_beef.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_crispy_taco = item!(
        id: ID::parse_str("6400e70c-0bce-4543-8be3-5cddd312a22f").unwrap(),
        long_name: "Crispy Taco Bean",
        short_name: "TACO-BN",
        price: 169, plu: "140",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                refried_beans.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )),
            default_item_ids: vec!(
                refried_beans.id,
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                hard_shell.id,
                lettuce.id,
                mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chipotle_chicken_taco_bravo = item!(
        id: ID::parse_str("648ef2fa-4f6c-448a-a286-e064bd3cb2b9").unwrap(),
        long_name: "Chipotle Chicken Taco Bravo",
        short_name: "T-BRV-CHP-CK",
        price: 315, plu: "57200",
        item_priority: 1,
        tags: hashset!("dinner", "lunch", "tacos"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Items,
                selection: ItemSelection::Tag("proteins".to_owned()),
                default_item_ids: vec!(chicken.id)
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    creamy_chipotle.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    creamy_chipotle.id,
                    lettuce.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    cheddar_cheese.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    creamy_chipotle.id,
                    lettuce.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    cheddar_cheese.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_taco_bravo = item!(
        id: ID::parse_str("65edba48-dc71-4f30-8e6c-2cd27b0f126d").unwrap(),
        long_name: "Steak Taco Bravo",
        short_name: "T-BRV-STK",
        price: 379, plu: "320",
        item_priority: 2,
        slots: vec!(

            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_taco_bravo = item!(
        id: ID::parse_str("67fe0cd3-dde9-44aa-92d9-5c0aefbc411f").unwrap(),
        long_name: "Beef Taco Bravo",
        short_name: "T-BRV-BF",
        price: 299, plu: "300",
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_taco_bravo = item!(
        id: ID::parse_str("686633cb-c89a-4672-be98-c1c536833632").unwrap(),
        long_name: "Chicken Taco Bravo",
        short_name: "T-BRV-CK",
        price: 315, plu: "310",
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_taco_bravo = item!(
        id: ID::parse_str("686a3caf-ad46-4216-92aa-90c5fa1fe046").unwrap(),
        long_name: "Bean Taco Bravo",
        short_name: "T-BRV-BN",
        price: 299, plu: "340",
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )),
                default_item_ids: vec!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    hard_shell.id,
                    small_tortilla.id,
                    refried_beans.id,
                    mild_sauce.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    tomato.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_taco_burger = item!(
        id: ID::parse_str("69fb9c82-92a6-4be9-adb1-41fec1d1724c").unwrap(),
        long_name: "Steak Taco Burger",
        short_name: "T-BURG-STK",
        price: 359, plu: "370",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_taco_burger = item!(
        id: ID::parse_str("6a0c0d61-fcdb-4953-857e-e80402fd4eea").unwrap(),
        long_name: "Beef Taco Burger",
        short_name: "T-BURG",
        price: 315, plu: "350",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_taco_burger = item!(
        id: ID::parse_str("6a8dcfc5-ebfa-45b7-a22e-35dd47a78189").unwrap(),
        long_name: "Chicken Taco Burger",
        short_name: "T-BURG-CK",
        price: 299, plu: "360",
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    burger_bun.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    chicken.id,
                    cheddar_cheese.id,
                    lettuce.id,
                    mild_sauce.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_stuffed_grilled_taco = item!(
        id: ID::parse_str("6af8bc53-af24-4e99-a8a5-c1b0b6ef1806").unwrap(),
        long_name: "Steak Stuffed Grilled Taco",
        short_name: "SGT-STK",
        price: 482, plu: "3000",
        tags: hashset!("grilled"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_stuffed_grilled_taco = item!(
        id: ID::parse_str("6b9e2f36-0ae5-4583-9dba-45e4d3caf370").unwrap(),
        long_name: "Beef Stuffed Grilled Taco",
        short_name: "SGT-BF",
        price: 370, plu: "2950",
        tags: hashset!("dinner","lunch","tacos", "grilled"),
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_stuffed_grilled_taco = item!(
        id: ID::parse_str("6cd3c609-697d-4c31-9e34-e5b9d79784e4").unwrap(),
        long_name: "Chicken Stuffed Grilled Taco",
        short_name: "SGT-CK",
        price: 409, plu: "57210",
        tags: hashset!("grilled"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_stuffed_grilled_taco = item!(
        id: ID::parse_str("a367ad43-8ceb-45bc-8896-19f59c019485").unwrap(),
        long_name: "Bean Stuffed Grilled Taco",
        short_name: "SGT-BN",
        price: 349, plu: "3030",
        tags: hashset!("grilled"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    hard_shell.id,
                    mild_sauce.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let stuffed_grilled_chipotle_chicken_taco = item!(
        id: ID::parse_str("6d00115b-e6f5-499a-aca9-9a31dce24915").unwrap(),
        long_name: "Stuffed Grilled Chipotle Chicken Taco",
        short_name: "STG-CHCK",
        price: 409, plu: "2960",
        tags: hashset!("dinner", "lunch", "tacos", "grilled"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Items,
                selection: ItemSelection::Id(chicken.id),
                default_item_ids: vec!(chicken.id)
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    small_tortilla.id,
                    hard_shell.id,
                    creamy_chipotle.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    creamy_chipotle.id,
                    small_tortilla.id,
                    hard_shell.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    creamy_chipotle.id,
                    small_tortilla.id,
                    hard_shell.id,
                    sour_cream.id,
                    nacho_cheese.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let citrus_chipotle_fried_chicken_softshell_taco = item!(
        id: ID::parse_str("6df6156d-c637-4485-925a-707e659e5c06").unwrap(),
        long_name: "CL FC Taco",
        short_name: "TACO-CL-FC",
        price: 249, plu: "12500",
        item_priority: 3,
        tags: hashset!("dinner", "lunch", "tacos", "quick"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::Id(fried_chicken.id),
                default_item_ids: vec!(fried_chicken.id),
                price_overrides: vec!(price_override!(tags: hashset!("proteins"), price: 0))
            ),
            // slot!(
            //     name: "Sauce",
            //     collapsed: false,
            //     slot_type: SlotType::Replace,
            //     selection: ItemSelection::AnyId(hashset!(
            //         chipotle_lime.id
            //     )),
            //     minimum_quantity: 1,
            //     maximum_quantity: 1
            // ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let spicy_jalapeno_ranch_fried_chicken_taco = item!(
        id: ID::parse_str("6e938317-0554-4212-9219-00add2016802").unwrap(),
        long_name: "SJR FC Taco",
        short_name: "TACO-JRFC",
        price: 249, plu: "12510",
        item_priority: 2,
        tags: hashset!("dinner", "lunch", "tacos", "quick"),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::Id(fried_chicken.id),
                default_item_ids: vec!(fried_chicken.id),
                price_overrides: vec!(price_override!(tags: hashset!("proteins"), price: 0))
            ),
            // slot!(
            //     name: "Sauce",
            //     collapsed: false,
            //     slot_type: SlotType::Replace,
            //     selection: ItemSelection::AnyId(hashset!(
            //         jalapeno_ranch.id
            //     )),
            //     minimum_quantity: 1,
            //     maximum_quantity: 1
            // ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_softshell_taco = item!(
        id: ID::parse_str("71b15f13-a65e-44e6-adc1-fe57c2830add").unwrap(),
        long_name: "Steak Softshell Taco",
        short_name: "SS-STK",
        price: 317, plu: "220",
        item_priority: 3,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_softshell_taco = item!(
        id: ID::parse_str("72843bf4-9c44-427b-8015-ea5ee49e0dd1").unwrap(),
        long_name: "Beef Softshell Taco",
        short_name: "SS-BF",
        price: 199, plu: "200",
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_softshell_taco = item!(
        id: ID::parse_str("7293c757-d3d5-4046-acbb-5967ea10ad50").unwrap(),
        long_name: "Bean Softshell Taco",
        short_name: "SS-BN",
        price: 199, plu: "240",
        item_priority: 4,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_softshell_taco = item!(
        id: ID::parse_str("74d36b2d-5edf-4c92-a8b2-a7ad92c184a5").unwrap(),
        long_name: "Chicken Softshell Taco",
        short_name: "SS-CK",
        price: 262, plu: "210",
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    small_tortilla.id,
                    mild_sauce.id,
                    lettuce.id,
                    cheddar_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_street_taco = item!(
        id: ID::parse_str("75785681-6cc7-4f1d-b023-c2e1048f45eb").unwrap(),
        long_name: "Steak Street Taco",
        short_name: "ST-STK",
        price: 299, plu: "6900",
        item_priority: 1,
        tags: hashset!("street_tacos", "value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    lettuce.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    lettuce.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_street_taco = item!(
        id: ID::parse_str("76c52e23-a0cc-4a18-b016-5d1b637067be").unwrap(),
        long_name: "Chicken Street Taco",
        short_name: "ST-CK",
        price: 240, plu: "7590",
        tags: hashset!("street_tacos", "value"),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    lettuce.id,
                    guacamole.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    lettuce.id,
                    guacamole.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    lettuce.id,
                    pico_de_gallo.id,
                    queso_fresco.id,
                    chimichurri.id,
                    corn_tortilla.id,
                    lime.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let street_taco_trio = item!(
        id: ID::parse_str("777c5f3e-6c5f-43b6-b768-fa2491956864").unwrap(),
        long_name: "Street Taco Trio",
        short_name: "ST-TRI",
        price: 859, plu: "6910",
        tags: hashset!("dinner", "lunch", "tacos"),
        slots: vec!(
            slot!(
                name: "Tacos",
                slot_type: SlotType::Items,
                selection: ItemSelection::AnyId(hashset!(steak_street_taco.id, chicken_street_taco.id)),
                minimum_quantity: 3,
                maximum_quantity: 3,
                price_overrides: vec!(price_override!(tags: hashset!("street_tacos"), price: 0))
            )
        )
    );

    let cinco_softshell = item!(
        id: ID::parse_str("785cd853-b1eb-4649-8e7f-664451def465").unwrap(),
        long_name: "Cinco Softshell",
        short_name: "CINCO-SS",
        price: 555, plu: "7500",
        tags: hashset!("dinner", "lunch"),
        slots: vec!(
            slot!(
                name: "Tacos",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset![beef_softshell_taco.id,beef_softshell_taco.id,beef_softshell_taco.id,beef_softshell_taco.id,beef_softshell_taco.id ]),
                minimum_quantity: 5,
                maximum_quantity: 5,
                default_quantity: 5,
                price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
                default_item_ids: vec![beef_softshell_taco.id; 5]
            )
        )
    );

    let bacon_and_egg_burrito = item!(
    id: ID::parse_str("79bb3918-8b3c-494f-b4b2-d97e9c4f4045").unwrap(),
    long_name: "Bacon M&E Burrito",
    short_name: "Bcn M&E Burr",
    price: 379, plu: "4010",
    slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                bacon.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )),
            default_item_ids: vec!(
                bacon.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
        ),
        dynamic_add_ons(
            vec!(
                bacon.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
            ,add_ons_vec.clone()
        )
        )
    );

    let sausage_and_egg_burrito = item!(
    id: ID::parse_str("e7786aeb-d7e1-46a9-b0f0-5a65661a23a6").unwrap(),
    long_name: "Sausage M&E Burrito",
    short_name: "Saus M&E Burr",
    price: 379, plu: "4020",
    slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                sausage.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )),
            default_item_ids: vec!(
                sausage.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
        ),
        dynamic_add_ons(
            vec!(
                sausage.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
            ,add_ons_vec.clone()
        )
        )
    );

    let steak_and_egg_burrito = item!(
    id: ID::parse_str("7a88c26e-a3ac-4f4d-9edf-e4ef3e93f6be").unwrap(),
    long_name: "Steak M&E Burrito",
    short_name: "Stk M&E Burr",
    price: 499, plu: "3050",
    slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                steak.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )),
            default_item_ids: vec!(
                steak.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
        ),
        dynamic_add_ons(
            vec!(
                steak.id,
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
            ,add_ons_vec.clone()
        )
        )
    );

    let grilled_burrito = item!(
        id: ID::parse_str("7aa2cb22-8c9e-484a-9b78-6dd03b831d12").unwrap(),
        long_name: "Grilled Burrito",
        short_name: "GRLD",
        price: 469, plu: "840",
        tags: hashset!("dinner", "lunch", "grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Protein",
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::Tag("proteins".to_owned()),
                default_item_ids: vec!(chicken.id),
                price_overrides: vec!(price_override!(tags: hashset!("proteins"), price: 0))
            ),
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let steak_grilled_burrito = item!(
        id: ID::parse_str("7b6ab5be-cd5a-40fe-bd93-f403080cee44").unwrap(),
        long_name: "Steak Grilled Burrito",
        short_name: "GRLD-STK",
        price: 589, plu: "820",
        tags: hashset!("grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    steak.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    steak.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let beef_grilled_burrito = item!(
        id: ID::parse_str("7bb48f46-4e09-4e9f-8472-1919c135af25").unwrap(),
        long_name: "Beef Grilled Burrito",
        short_name: "GRLD-BF",
        price: 499, plu: "800",
        tags: hashset!("grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    ground_beef.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    ground_beef.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let chicken_grilled_burrito = item!(
        id: ID::parse_str("7e0dc31a-27e6-40b8-a2e1-81eaa2c1b827").unwrap(),
        long_name: "Chicken Grilled Burrito",
        short_name: "GRLD-CK",
        price: 529, plu: "810",
        tags: hashset!("grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    chicken.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    chicken.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let fried_chicken_grilled_burrito = item!(
        id: ID::parse_str("ebb99831-1e87-4c6e-88d8-20646c920680").unwrap(),
        long_name: "FC Grilled Burrito",
        short_name: "GRLD-FC",
        price: 529, plu: "811",
        tags: hashset!("grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    fried_chicken.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    fried_chicken.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let bean_grilled_burrito = item!(
        id: ID::parse_str("c3ee9a89-3271-4bab-908c-b40f1a05bccc").unwrap(),
        long_name: "Bean Grilled Burrito",
        short_name: "GRLD-BN",
        price: 469, plu: "840",
        tags: hashset!("grilled"),
        item_priority: 1,
        modifiers: vec!(Modification::Custom("Not Grilled".to_string())),
        slots: vec!(
            slot!(
                name: "Ingredients",
                collapsed: false,
                slot_type: SlotType::Ingredient,
                selection: ItemSelection::AnyId(hashset!(
                    refried_beans.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )),
                default_item_ids: vec!(
                    refried_beans.id,
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
            ),
            dynamic_add_ons(
                vec!(
                    potato_ole.id,
                    side_ranch.id,
                    cheddar_cheese.id,
                    mild_sauce.id,
                    nacho_cheese.id
                )
                ,add_ons_vec.clone()
            )
        )
    );

    let _fried_chicken_five_piece = item!(
        id: ID::parse_str("7d44a69e-fb06-4a90-a083-ea16b8e77039").unwrap(),
        long_name: "Fried Chicken 5 Piece",
        short_name: "FC",
        price: 0, plu: "12720",
        tags: hashset!("dinner", "lunch"),
        slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                chicken.id,
                side_ranch.id
            )),
            default_item_ids: vec!(
                chicken.id,
                side_ranch.id
            )
        ),
        dynamic_add_ons(
            vec!(
                chicken.id,
                side_ranch.id
            )
            ,add_ons_vec.clone()
        )
    )
    );

    let combo_size_variations = vec![
        variation!( name: "Small", identifier: "SM"),
        variation!( name: "Medium", identifier: "MD"),
        variation!( name: "Large", identifier: "LG"),
    ];

    let kids_meal_size_variations =
        vec![variation!( name: "Kid", prefix: Some(String::from("KD")), identifier: "64")];

    let med_large_variations = vec![
        variation!( name: "Medium", prefix: Some(String::from("MD")), identifier: "128"),
        variation!( name: "Large", prefix: Some(String::from("LG")), identifier: "256"),
    ];

    let kids_sides = slot!(
        name: "Sides",
        slot_type: SlotType::Replace,
        selection: ItemSelection::AnyId(hashset!(potato_ole_side.id)),
        price_overrides: vec!(
            price_override!(tags: hashset!("sides"), price: 0, variation: Some("Kid".into())),
        ),
        default_item_ids: vec!(
            potato_ole_side.id
        )
    );

    let boss_bowl_shell_item = item!(
        id: ID::parse_str("7e40f41f-6b67-4ed7-bdc0-df06383ef14c").unwrap(),
        long_name: "Boss Bowl",
        short_name: "B-BWL",
        tags: hashset!("dinner", "lunch", "specialties"),
        item_priority: 6,
        slots: vec!(
            slot!(
                name: "Boss Bowl",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_boss_bowl.id, chicken_boss_bowl.id, steak_boss_bowl.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_boss_bowl.id)
            ),
        )
    );

    let burrito_shell_item = item!(
        id: ID::parse_str("7f5e5082-75aa-4522-93c0-8f0319fbd5bb").unwrap(),
        long_name: "Burrito",
        short_name: "BUR",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_burrito.id, chicken_burrito.id, fried_chicken_burrito.id, steak_burrito.id, bean_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_burrito.id)
            ),
        )
    );

    let meat_and_egg_burrito_shell_item = item!(
        id: ID::parse_str("7fad5929-18e8-4486-ac99-3a74de3460dd").unwrap(),
        long_name: "Meat and Egg Burrito",
        short_name: "ME-BUR",
        tags: hashset!("dinner", "lunch", "breakfast"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Meat and Egg Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(bacon_and_egg_burrito.id, sausage_and_egg_burrito.id, steak_and_egg_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(bacon_and_egg_burrito.id)
            ),
        )
    );

    let boss_burrito_shell_item = item!(
    id: ID::parse_str("e872b650-6bb5-4135-8406-180afef31bd5").unwrap(),
        long_name: "Boss Burrito",
        short_name: "BUR-BOSS",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Boss Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_boss_burrito.id, chicken_boss_burrito.id, steak_boss_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_boss_burrito.id)
            ),
        )
    );

    let combination_burrito_shell_item = item!(
    id: ID::parse_str("e887ef09-27d1-46c0-86de-1809f0401f8f").unwrap(),
        long_name: "Combination Burrito",
        short_name: "BUR-CB",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Combination Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::Id(combination_burrito.id),
                minimum_quantity: 1,
                default_item_ids: vec!(combination_burrito.id)
            ),
        )
    );

    let grilled_burrito_shell_item = item!(
        id: ID::parse_str("80212043-75f0-49dc-8a20-0a8cc876cdbe").unwrap(),
        long_name: "Grilled Burrito",
        short_name: "BUR-GRLD",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Grilled Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_grilled_burrito.id, chicken_grilled_burrito.id, steak_grilled_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_grilled_burrito.id)
            ),
        )
    );

    let meat_and_potato_burrito_shell_item = item!(
        id: ID::parse_str("80bfb7b8-b52f-46a1-82bf-d31aa6072103").unwrap(),
        long_name: "Meat and Potato Burrito",
        short_name: "MPB",
        tags: hashset!("dinner", "lunch", "burrito", "quick"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Meat and Potato Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_meat_potato_burrito.id, chicken_meat_potato_burrito.id, fried_chicken_meat_potato_burrito.id, steak_meat_potato_burrito.id, bean_meat_potato_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_meat_potato_burrito.id)
            ),
        )
    );

    let super_burrito_shell_item = item!(
        id: ID::parse_str("8127e5d7-2836-4018-bf4c-7af1bb241d36").unwrap(),
        long_name: "Super Burrito",
        short_name: "BUR-SUP",
        tags: hashset!("dinner", "lunch", "burrito"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Super Burrito",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_super_burrito.id, chicken_super_burrito.id, steak_super_burrito.id, bean_super_burrito.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_super_burrito.id)
            ),
        )
    );

    let quesadilla_shell_item = item!(
        id: ID::parse_str("8172e1ca-4bb5-4448-a914-61657b380ab5").unwrap(),
        long_name: "Quesadilla",
        short_name: "QUES",
        tags: hashset!("dinner", "lunch", "quick","specialties"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Quesadilla",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(quesadilla.id, beef_quesadilla.id, chicken_quesadilla.id, steak_quesadilla.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(quesadilla.id)
            ),
        )
    );

    let super_nachos_shell_item = item!(
        id: ID::parse_str("81eff678-8194-4009-83f3-304f75b63e00").unwrap(),
        long_name: "Super Nachos",
        short_name: "SN",
        tags: hashset!("dinner", "lunch", "specialties"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Super Nachos",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_super_nachos.id, chicken_super_nachos.id, steak_super_nachos.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_super_nachos.id)
            ),
        )
    );

    let fried_chicken_taco_shell_item = item!(
        id: ID::parse_str("47566a7a-3ee0-42f6-9ae0-cc93c1434c75").unwrap(),
        long_name: "Fried Chicken Taco",
        short_name: "T-FC",
        tags: hashset!("dinner", "lunch", "tacos"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Fried Chicken Taco",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(citrus_chipotle_fried_chicken_softshell_taco.id, spicy_jalapeno_ranch_fried_chicken_taco.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(citrus_chipotle_fried_chicken_softshell_taco.id)
            ),
        )
    );

    let taco_bravo_shell_item = item!(
        id: ID::parse_str("f2173e5d-a601-43f0-84e5-3d7c0325958c").unwrap(),
        long_name: "Taco Bravo",
        short_name: "T-BRG",
        tags: hashset!("dinner", "lunch", "tacos", "quick"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Taco Bravo",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_taco_bravo.id, chicken_taco_bravo.id, steak_taco_bravo.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_taco_bravo.id)
            ),
        )
    );

    let taco_burger_shell_item = item!(
        id: ID::parse_str("832c8ca1-69be-4355-b2a2-07eec12be90c").unwrap(),
        long_name: "Taco Burger",
        short_name: "T-BUR",
        tags: hashset!("dinner", "lunch", "tacos"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Taco Burger",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_taco_burger.id, chicken_taco_burger.id, steak_taco_burger.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_taco_burger.id)
            ),
        )
    );

    let taco_shell_item = item!(
        id: ID::parse_str("8504a103-e3dc-4085-af7b-107b210bb94b").unwrap(),
        long_name: "Crispy Taco",
        short_name: "Taco",
        tags: hashset!("dinner", "lunch", "tacos", "quick"),
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Taco",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, chicken_crispy_taco.id, steak_crispy_taco.id, bean_crispy_taco.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_crispy_taco.id)
            ),
        )
    );

    let softshell_shell_item = item!(
        id: ID::parse_str("85715cc6-74db-4194-9855-4f3b0b888ee3").unwrap(),
        long_name: "Softshell",
        short_name: "SS",
        tags: hashset!("dinner", "lunch", "tacos", "quick"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Softshell",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id, chicken_softshell_taco.id, steak_softshell_taco.id, bean_softshell_taco.id)),
                minimum_quantity: 1,
                default_item_ids: vec!(beef_softshell_taco.id)
            ),
        )
    );

    let _crispy_taco_combo = item!(
        id: ID::parse_str("85cbfd7e-28c5-4f54-a03f-3d047047d74f").unwrap(),
        long_name: "2 Crispy Taco Combo",
        short_name: "#1",
        label: Some("#1".to_string()),
        price: 720, plu: "1000",
        item_priority: 3,
        tags: hashset!("dinner", "lunch", "combos", "quick", "bundle"),
        variations: combo_size_variations.clone(),
        default_variation: combo_size_variations[0].id,
        slots: vec!(
            slot!(
                name: "Entree",
                slot_type: SlotType::Items,
                selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, chicken_crispy_taco.id, steak_crispy_taco.id, bean_crispy_taco.id)),
                minimum_quantity: 2,
                maximum_quantity: 2,
                default_quantity: 2,
                price_overrides: vec!(price_override!(item_ids: hashset!(beef_crispy_taco.id, bean_crispy_taco.id), price: 0), price_override!(item_ids: hashset!(chicken_crispy_taco.id), price: 29), price_override!(item_ids: hashset!(steak_crispy_taco.id), price: 54))
            ),
            sides.clone(),
            drinks.clone(),
        )
    );

    let _super_burrito_combo = item!(
    id: ID::parse_str("e9715ae8-012c-4221-8995-18084c5425f6").unwrap(),
    long_name: "Super Burrito Combo",
    short_name: "#2",
    label: Some("#2".to_string()),
    price: 739, plu: "1100",
    item_priority: 3,
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_super_burrito.id, chicken_super_burrito.id, steak_super_burrito.id, bean_super_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(beef_super_burrito.id, bean_super_burrito.id), price: 0), price_override!(item_ids: hashset!(chicken_super_burrito.id), price: 10), price_override!(item_ids: hashset!(steak_super_burrito.id), price: 108))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _boss_burrito_combo = item!(
    id: ID::parse_str("e9fa5ab7-5f38-4bab-910c-afaed91764d3").unwrap(),
    long_name: "Boss Burrito Combo",
    short_name: "Boss Burr Cmb",
    label: Some("#10".to_string()),
    price: 999, plu: "12400",
    item_priority: 2,
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(chicken_boss_burrito.id, steak_boss_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(steak_boss_burrito.id.clone()),
            price_overrides: vec!(price_override!(item_ids: hashset!(chicken_boss_burrito.id), price: 0), price_override!(item_ids: hashset!(steak_boss_burrito.id), price: 50))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _fried_chicken_combo = item!(
    id: ID::parse_str("69aec63e-ebdb-476d-8857-dbbdf0c518f9").unwrap(),
    long_name: "Fried Chicken Combo",
    short_name: "#11",
    label: Some("#11".to_string()),
    price: 799, plu: "12580",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    item_priority: 2,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
            slot!(
                name: "Entree",
                slot_type: SlotType::Items,
                selection: ItemSelection::AnyId(hashset!(citrus_chipotle_fried_chicken_softshell_taco.id, spicy_jalapeno_ranch_fried_chicken_taco.id)),
                minimum_quantity: 2,
                maximum_quantity: 2,
                default_quantity: 2,
                //default_item_ids: vec!(citrus_chipotle_fried_chicken_softshell_taco.id),
                price_overrides: vec!(price_override!(item_ids: hashset!(citrus_chipotle_fried_chicken_softshell_taco.id, spicy_jalapeno_ranch_fried_chicken_taco.id), price: 0))
            ),
            sides.clone(),
            drinks.clone()
        )
    );

    let _two_softshells_combo = item!(
        id: ID::parse_str("85f6310e-aed5-4773-ae86-9e63d652ea29").unwrap(),
        long_name: "2 SS Taco Combo",
        short_name: "#3",
        label: Some("#3".to_string()),
        price: 765, plu: "1200",
        tags: hashset!("dinner", "lunch", "combos", "quick", "bundle"),
        item_priority: 3,
        variations: combo_size_variations.clone(),
        default_variation: combo_size_variations[0].id,
        slots: vec!(
            slot!(
                name: "Entree",
                slot_type: SlotType::Items,
                selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id, chicken_softshell_taco.id, steak_softshell_taco.id, bean_softshell_taco.id)),
                minimum_quantity: 2,
                maximum_quantity: 2,
                default_quantity: 2,
                price_overrides: vec!(price_override!(item_ids: hashset!(beef_softshell_taco.id, bean_softshell_taco.id), price: 0), price_override!(item_ids: hashset!(chicken_softshell_taco.id), price: 37), price_override!(item_ids: hashset!(steak_softshell_taco.id), price: 134))
            ),
            sides.clone(),
            drinks.clone()
        )
    );

    let _taco_bravo_combo = item!(
    id: ID::parse_str("ebcb411a-a698-4e46-aea5-2295086f8fd1").unwrap(),
    long_name: "Taco Bravo Combo",
    short_name: "#4",
    label: Some("#4".to_string()),
    price: 679, plu: "1300",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    item_priority: 3,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_taco_bravo.id,steak_taco_bravo.id, chicken_taco_bravo.id, bean_taco_bravo.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(beef_taco_bravo.id),
            price_overrides: vec!(price_override!(item_ids: hashset!(beef_taco_bravo.id, bean_taco_bravo.id), price: 0), price_override!(item_ids: hashset!(chicken_taco_bravo.id), price: 20), price_override!(item_ids: hashset!(steak_taco_bravo.id), price: 50))
            ),
            sides.clone(),
            drinks.clone()
        )
    );

    let _kids_meal = item!(
    id: ID::parse_str("ec359191-3352-4c02-91fd-bd5845f13f5e").unwrap(),
    long_name: "Kids Meal Crispy Taco",
    short_name: "KM-TACO",
    price: 479, plu: "1900",
    tags: hashset!("dinner", "lunch", "kids_meal"),
    item_priority: 6,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, steak_crispy_taco.id, chicken_crispy_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(beef_crispy_taco.id.clone()),
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0))
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _kids_meal_soft = item!(
    id: ID::parse_str("ec7be9fd-bd29-4951-a689-f81d7bb5f98a").unwrap(),
    long_name: "Kids Meal Soft Taco",
    short_name: "KM-SS",
    price: 479, plu: "1910",
    tags: hashset!("dinner", "lunch","kids_meal"),
    item_priority: 5,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id, steak_softshell_taco.id, chicken_softshell_taco.id, bean_softshell_taco.id, citrus_chipotle_fried_chicken_softshell_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(beef_softshell_taco.id.clone()),
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0))
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _kids_meal_burr = item!(
    id: ID::parse_str("ee77484c-f3da-494f-8347-a26ae6e7ed46").unwrap(),
    long_name: "Kids Meal Burrito",
    short_name: "KM-BUR",
    price: 479, plu: "1920",
    tags: hashset!("dinner", "lunch", "kids_meal"),
    item_priority: 3,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_burrito.id, steak_burrito.id, chicken_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(beef_burrito.id),
            price_overrides: vec!(price_override!(tags: hashset!("burrito"), price: 0))
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _kids_meal_taco_burger = item!(
    id: ID::parse_str("eecc8f3c-51e8-45ca-ab48-b337c18c2d96").unwrap(),
    long_name: "Kids Meal Taco Burger",
    short_name: "KM-TBURG",
    price: 529, plu: "1930",
    tags: hashset!("dinner", "lunch", "kids_meal"),
    item_priority: 1,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_taco_burger.id, chicken_taco_burger.id, steak_taco_burger.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(beef_taco_burger.id),
            price_overrides: vec!(price_override!(item_ids: hashset!(beef_taco_burger.id, chicken_taco_burger.id, steak_taco_burger.id), price: 0))
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _kids_meal_fried_chicken = item!(
    id: ID::parse_str("f016131a-5af2-48a5-b9be-605d8094c8cd").unwrap(),
    long_name: "Kids Meal Fried Chicken",
    short_name: "KM-FC",
    price: 549, plu: "12560",
    tags: hashset!("dinner", "lunch", "kids_meal"),
    item_priority: 4,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: true,
            selection: ItemSelection::AnyId(hashset!(fried_chicken.id.clone())),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
            default_item_ids: vec!(fried_chicken.id.clone())
            ),
            slot!(
                name: "Sauce",
                slot_type: SlotType::Replace,
                maximum_quantity: 1,
                free_quantity: 1,
                selection: ItemSelection::AnyId(hashset!(ranch.id, jalapeno_ranch.id, chipotle_lime.id))
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _kids_meal_quesadilla = item!(
    id: ID::parse_str("f052eabb-b797-440f-a323-1a379ae74277").unwrap(),
    long_name: "Kids Meal Quesadilla",
    short_name: "KM-QUES",
    price: 479, plu: "1950",
    tags: hashset!("dinner", "lunch", "kids_meal"),
    item_priority: 2,
    variations: kids_meal_size_variations.clone(),
    default_variation: kids_meal_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: true,
            selection: ItemSelection::AnyId(hashset!(snack_cheese_quesadilla.id.clone())),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(snack_cheese_quesadilla.id.clone())
            ),
            kids_sides.clone(),
            kids_beverages.clone()
        )
    );

    let _meat_potato_burrito_combo = item!(
    id: ID::parse_str("f076d8f1-7a05-4417-807d-a2366f7cb163").unwrap(),
    long_name: "M&P Burrito Combo",
    short_name: "#5",
    label: Some("#5".to_string()),
    price:  755, plu: "1400",
    tags: hashset!("dinner", "lunch", "combos", "quick", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 3,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_meat_potato_burrito.id, chicken_meat_potato_burrito.id, fried_chicken_meat_potato_burrito.id, steak_meat_potato_burrito.id, bean_meat_potato_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(
                price_override!(item_ids: hashset!(beef_meat_potato_burrito.id), price: 0),
                price_override!(item_ids: hashset!(bean_meat_potato_burrito.id), price: -36),
                price_override!(item_ids: hashset!(chicken_meat_potato_burrito.id), price: 84),
                price_override!(item_ids: hashset!(fried_chicken_meat_potato_burrito.id), price: 44),
                price_override!(item_ids: hashset!(steak_meat_potato_burrito.id), price: 144)
            )
        ),
        sides.clone(),
        drinks.clone(),
        )
    );

    let _grilled_burrito_combo = item!(
    id: ID::parse_str("f173cc55-589f-4d66-bfd7-f6152e232ae9").unwrap(),
    long_name: "Grilled Burrito Combo",
    short_name: "#6",
    label: Some("#6".to_string()),
    price:  815, plu: "1500",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 3,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(chicken_grilled_burrito.id, fried_chicken_grilled_burrito.id, beef_grilled_burrito.id, steak_grilled_burrito.id, bean_grilled_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(chicken_grilled_burrito.id),
            price_overrides: vec!(
                price_override!(item_ids:hashset!(beef_grilled_burrito.id), price: 0),
                price_override!(item_ids:hashset!(bean_grilled_burrito.id), price: -76),
                price_override!(item_ids:hashset!(steak_grilled_burrito.id), price: 44),
                price_override!(item_ids:hashset!(chicken_grilled_burrito.id, fried_chicken_grilled_burrito.id), price: 24),
            )
        ),
        sides.clone(),
        drinks.clone(),
        )
    );

    let _street_taco_trio_combo = item!(
    id: ID::parse_str("f2cb379d-c138-4320-a104-49baa08d4dc2").unwrap(),
    long_name: "Street Taco Trio Combo",
    short_name: "#7",
    label: Some("#7".to_string()),
    price: 979, plu: "6920",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    item_priority: 3,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Tacos",
            slot_type: SlotType::Items,
            selection: ItemSelection::AnyId(hashset!(steak_street_taco.id, chicken_street_taco.id)),
            minimum_quantity: 3,
            maximum_quantity: 3,
            price_overrides: vec!(price_override!(item_ids: hashset!(chicken_street_taco.id), price: 0), price_override!(item_ids: hashset!(steak_street_taco.id), price: 30))
        ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _meat_and_potato_burrito_and_softshell_combo = item!(
        id: ID::parse_str("f2dca75d-0b8c-487a-908e-2344410b3526").unwrap(),
        long_name: "M&P & SS Taco Combo",
        short_name: "#8",
        label: Some("#8".to_string()),
        price:  935, plu: "1700",
        tags: hashset!("dinner", "lunch", "combos", "bundle"),
        variations: combo_size_variations.clone(),
        default_variation: combo_size_variations[0].id,
        item_priority: 3,
        slots: vec!(
            slot!(
                name: "Burrito",
                slot_type: SlotType::Replace,
                selection: ItemSelection::AnyId(hashset!(beef_meat_potato_burrito.id, chicken_meat_potato_burrito.id, steak_meat_potato_burrito.id, fried_chicken_meat_potato_burrito.id, bean_meat_potato_burrito.id)),
                minimum_quantity: 1,
                maximum_quantity: 1,
                price_overrides: vec!(
                    price_override!(item_ids: hashset!(beef_meat_potato_burrito.id, bean_meat_potato_burrito.id, fried_chicken_meat_potato_burrito.id), price: 0),
                    price_override!(item_ids: hashset!(chicken_meat_potato_burrito.id), price: 50),
                    price_override!(item_ids: hashset!(steak_meat_potato_burrito.id), price: 100)
                )
            ),
            slot!(
                name: "Taco",
                slot_type: SlotType::Replace,
                selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, beef_softshell_taco.id, chicken_crispy_taco.id, chicken_softshell_taco.id, steak_crispy_taco.id, steak_softshell_taco.id, bean_crispy_taco.id, bean_softshell_taco.id)),
                minimum_quantity: 1,
                maximum_quantity: 1,
                price_overrides: vec!(
                    price_override!(item_ids: hashset!(beef_crispy_taco.id, beef_softshell_taco.id, bean_crispy_taco.id, bean_softshell_taco.id), price: 0),
                    price_override!(item_ids: hashset!(steak_crispy_taco.id, steak_softshell_taco.id), price: 100),
                    price_override!(item_ids: hashset!(chicken_crispy_taco.id, chicken_softshell_taco.id), price: 50)
                )
            ),
            sides.clone(),
            drinks.clone(),
        )
    );

    let _quesadilla_combo = item!(
    id: ID::parse_str("f2fce470-d3dc-4af8-b2fb-4c341f1a6b40").unwrap(),
    long_name: "Quesadilla Combo",
    short_name: "Ques Cmb",
    price:  688, plu: "1600",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 2,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(quesadilla.id, steak_quesadilla.id, beef_quesadilla.id, chicken_quesadilla.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(quesadilla.id, steak_quesadilla.id, beef_quesadilla.id, chicken_quesadilla.id), price: 0))
        ),
        sides.clone(),
        drinks.clone(),
        )
    );

    let _stuffed_grilled_taco_combo = item!(
    id: ID::parse_str("f3f079fb-8475-4de7-9ed7-7688de6a9110").unwrap(),
    long_name: "Stuffed Grilled Taco Combo",
    short_name: "#9",
    label: Some("#9".to_string()),
    price:  719, plu: "2970",
    tags: hashset!("dinner", "lunch", "combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 3,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_stuffed_grilled_taco.id, stuffed_grilled_chipotle_chicken_taco.id, steak_stuffed_grilled_taco.id, bean_stuffed_grilled_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(
                price_override!(item_ids: hashset!(steak_stuffed_grilled_taco.id), price: 96),
                price_override!(item_ids: hashset!(stuffed_grilled_chipotle_chicken_taco.id), price: 10),
                price_override!(item_ids: hashset!(beef_stuffed_grilled_taco.id, bean_stuffed_grilled_taco.id), price: 0)
            ),
            default_item_ids: vec!(
                beef_stuffed_grilled_taco.id.clone()
            )
        ),
        sides.clone(),
        drinks.clone(),
        )
    );

    let _ranch_burrito = item!(
    id: ID::parse_str("f4951f8f-e959-4e24-a56e-245651dc22fe").unwrap(),
    long_name: "Ranch Burrito",
    short_name: "Rch Burr",
    price: 419, plu: "3900",
    tags: hashset!("lunch", "dinner", "burrito"),
    slots: vec!(
        taco_proteins.clone(),
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                cheddar_cheese.id,
                mild_sauce.id,
                large_tortilla.id,
                lettuce.id,
                onions.id,
                tomato.id,
                side_ranch.id
            )),
            default_item_ids: vec!(
                cheddar_cheese.id,
                mild_sauce.id,
                large_tortilla.id,
                lettuce.id,
                onions.id,
                tomato.id,
                side_ranch.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                mild_sauce.id,
                large_tortilla.id,
                lettuce.id,
                onions.id,
                tomato.id,
                side_ranch.id
            )
            ,add_ons_vec.clone()
        )
        )
    );

    let _egg_burrito = item!(
    id: ID::parse_str("f4f87c59-4543-47a2-b96b-3126d3011d26").unwrap(),
    long_name: "Egg Burrito",
    short_name: "Egg Burr",
    price: 299, plu: "4000",
    tags: hashset!("breakfast"),
    item_priority: 15,
    slots: vec!(
        slot!(
            name: "Ingredients",
            collapsed: false,
            slot_type: SlotType::Ingredient,
            selection: ItemSelection::AnyId(hashset!(
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )),
            default_item_ids: vec!(
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
        ),
        dynamic_add_ons(
            vec!(
                cheddar_cheese.id,
                salsa.id,
                large_tortilla.id,
                scrambled_eggs.id
            )
            ,add_ons_vec.clone()
        )
        )
    );

    let family_pack = item!(
        id: ID::parse_str("b2181628-9a77-4070-a3ae-5a804bab1ab7").unwrap(),
        long_name: "Family Pack",
        short_name: "Family Pack",
        price: 1600, plu: "10800",
        tags: hashset!("dinner", "lunch", "combos", "bundle"),
        item_priority: 1,
        slots: vec!(
            slot!(
                name: "Tacos",
                slot_type: SlotType::Items,
                collapsed: false,
                selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id, beef_crispy_taco.id)),
                minimum_quantity: 4,
                maximum_quantity: 4,
                default_quantity: 4,
                price_overrides: vec!(price_override!(item_ids: hashset!(beef_softshell_taco.id, beef_crispy_taco.id), price: 0))
            ),
            slot!(
                name: "Burritos",
                slot_type: SlotType::Items,
                collapsed: false,
                selection: ItemSelection::AnyId(hashset!(beef_meat_potato_burrito.id)),
                minimum_quantity: 2,
                maximum_quantity: 2,
                default_quantity: 2,
                price_overrides: vec!(price_override!(item_ids: hashset!(beef_meat_potato_burrito.id), price: 0))
            ),
            slot!(
                name: "Side",
                slot_type: SlotType::Items,
                collapsed: true,
                selection: ItemSelection::AnyId(hashset![potato_ole_side.id, potato_ole_side.id]),
                minimum_quantity: 2,
                maximum_quantity: 2,
                default_quantity: 2,
                price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
                default_item_ids: vec![potato_ole_side.id; 2]
            )
        )
    );

    let six_pack_and_pound_combo_mix = item!(
    id: ID::parse_str("f516cb4a-4403-4090-9cca-28eafe384044").unwrap(),
    long_name: "Six Pack Mixed",
    short_name: "Six Mix",
    price: 1399, plu: "3300",
    tags: hashset!("dinner", "lunch", "bundle"),
    item_priority: 2,
    variations: med_large_variations.clone(),
    default_variation: med_large_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Items,
            collapsed: false,
            selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id, beef_crispy_taco.id, chicken_crispy_taco.id, chicken_softshell_taco.id, steak_crispy_taco.id, steak_softshell_taco.id)),
            minimum_quantity: 6,
            maximum_quantity: 6,
            default_quantity: 6,
            price_overrides: vec!(
                price_override!(item_ids: hashset!(beef_crispy_taco.id, bean_crispy_taco.id, beef_softshell_taco.id, bean_softshell_taco.id), price: 0),
                price_override!(item_ids: hashset!(chicken_crispy_taco.id, chicken_softshell_taco.id), price: 30),
                price_override!(item_ids: hashset!(steak_crispy_taco.id, steak_softshell_taco.id), price: 55),
            )
        ),
        slot!(
            name: "Side",
            slot_type: SlotType::Items,
            collapsed: true,
            selection: ItemSelection::AnyId(hashset![potato_ole_side.id, potato_ole_side.id]),
            minimum_quantity: 2,
            maximum_quantity: 2,
            default_quantity: 2,
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
            default_item_ids: vec![potato_ole_side.id; 2]
            ),
        )
    );

    let six_pack_and_pound_combo_crispy = item!(
    id: ID::parse_str("f6171f2b-03c6-4c9e-9b88-3e01d968b309").unwrap(),
    long_name: "Six Pack Crispy",
    short_name: "Six Crispy",
    price: 1399, plu: "3290",
    tags: hashset!("dinner", "lunch", "bundle"),
    item_priority: 2,
    variations: med_large_variations.clone(),
    default_variation: med_large_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Items,
            collapsed: false,
            //selection: ItemSelection::AnyId(hashset![beef_crispy_taco.id, beef_crispy_taco.id, beef_crispy_taco.id, beef_crispy_taco.id, beef_crispy_taco.id, beef_crispy_taco.id]),
            selection: ItemSelection::AnyId(hashset![beef_crispy_taco.id, chicken_crispy_taco.id, steak_crispy_taco.id]),
            minimum_quantity: 6,
            maximum_quantity: 6,
            default_quantity: 6,
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
            default_item_ids: vec![beef_crispy_taco.id; 6]
            ),
        slot!(
            name: "Side",
            slot_type: SlotType::Items,
            collapsed: true,
            selection: ItemSelection::AnyId(hashset![potato_ole_side.id, potato_ole_side.id]),
            minimum_quantity: 2,
            maximum_quantity: 2,
            default_quantity: 2,
            default_item_ids: vec![potato_ole_side.id; 2]
            ),
        )
    );

    let six_pack_and_pound_combo_soft = item!(
    id: ID::parse_str("f6b37992-ff7e-4bde-a106-69456e69af7e").unwrap(),
    long_name: "Six Pack Soft",
    short_name: "Six Soft",
    price: 1399, plu: "3280",
    tags: hashset!("dinner", "lunch", "bundle"),
    item_priority: 2,
    variations: med_large_variations.clone(),
    default_variation: med_large_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Items,
            collapsed: false,
            selection: ItemSelection::AnyId(hashset![beef_softshell_taco.id, chicken_softshell_taco.id, steak_softshell_taco.id]),
            minimum_quantity: 6,
            maximum_quantity: 6,
            default_quantity: 6,
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
            default_item_ids: vec![beef_softshell_taco.id; 6]
            ),
        slot!(
            name: "Side",
            slot_type: SlotType::Items,
            collapsed: true,
            selection: ItemSelection::AnyId(hashset![potato_ole_side.id, potato_ole_side.id]),
            minimum_quantity: 2,
            maximum_quantity: 2,
            default_quantity: 2,
            price_overrides: vec!(price_override!(tags: hashset!("tacos"), price: 0)),
            default_item_ids: vec![potato_ole_side.id; 2]
            ),
        )
    );

    let six_pack_shell_item = item!(
        id: ID::parse_str("860dd6a9-608c-4a5b-815a-c88abdd77038").unwrap(),
        long_name: "Six Pack",
        short_name: "Six Pack",
        tags: hashset!("dinner", "lunch", "combos"),
        item_priority: 2,
        slots: vec!(
            slot!(
                name: "Style",
                slot_type: SlotType::ItemShell,
                selection: ItemSelection::AnyId(hashset!(six_pack_and_pound_combo_crispy.id, six_pack_and_pound_combo_soft.id, six_pack_and_pound_combo_mix.id)),
                minimum_quantity: 1
            ),
        )
    );

    let _meat_potato_breakfast_burrito_combo = item!(
    id: ID::parse_str("f6d0260d-d899-46d2-b359-0e6b85b5dd5f").unwrap(),
    long_name: "M&P Breakfast Burrito Combo",
    short_name: "B1-MPB",
    label: Some("#B1".to_string()),
    price: 639, plu: "3100",
    tags: hashset!("breakfast_combo", "quick", "bundle"),
    item_priority: 100,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(beef_potato_breakfast_burrito.id, bacon_potato_breakfast_burrito.id, sausage_potato_breakfast_burrito.id, steak_potato_breakfast_burrito.id, saus_and_bacon_meat_potato_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(
                price_override!(item_ids: hashset!(beef_potato_breakfast_burrito.id, bacon_potato_breakfast_burrito.id, sausage_potato_breakfast_burrito.id, saus_and_bacon_meat_potato_burrito.id), price: 0),
                price_override!(item_ids: hashset!(steak_potato_breakfast_burrito.id), price: 126)
            )
        ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _egg_burrito_combo = item!(
    id: ID::parse_str("f87a8da8-cb90-4602-944d-67b9fb4f8ab4").unwrap(),
    long_name: "Egg Breakfast Burrito Combo",
    short_name: "Egg Bk Br Cmb",
    price: 499, plu: "4260",
    tags: hashset!("breakfast_combo", "bundle"),
    item_priority: 90,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: true,
            selection: ItemSelection::Id(_egg_burrito.id),
            default_item_ids: vec!(_egg_burrito.id),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(_egg_burrito.id), price: 0))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _meat_egg_burrito_combo = item!(
    id: ID::parse_str("f892c1a5-901b-4a6a-bc9d-5be763e53aa5").unwrap(),
    long_name: "Meat Egg Breakfast Burrito Combo",
    short_name: "Mt Egg Bk Br Cmb",
    price: 529, plu: "4120",
    tags: hashset!("breakfast_combo", "bundle"),
    item_priority: 89,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(bacon_and_egg_burrito.id, sausage_and_egg_burrito.id, steak_and_egg_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(bacon_and_egg_burrito.id, sausage_and_egg_burrito.id, steak_and_egg_burrito.id), price: 0))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _triple_meat_potato_breakfast_burrito_combo = item!(
    id: ID::parse_str("f8c664c7-12e9-4e0c-ad1b-e7d4223ff6bf").unwrap(),
    long_name: "Triple M&P Breakfast Burrito Combo",
    short_name: "Trp M&P Bk Br Cmb",
    price: 699, plu: "6440",
    tags: hashset!("breakfast_combo", "bundle"),
    item_priority: 88,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: false,
            selection: ItemSelection::Id(triple_meat_potato_breakfast_burrito.id),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            default_item_ids: vec!(triple_meat_potato_breakfast_burrito.id),
            price_overrides: vec!(price_override!(item_ids: hashset!(triple_meat_potato_breakfast_burrito.id), price: 0))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _junior_breakfast_burrito_combo = item!(
    id: ID::parse_str("faa88261-36f3-40b0-870b-78a739a56049").unwrap(),
    long_name: "Jr. Breakfast Burrito Combo",
    short_name: "B2-JBUR",
    label: Some("#B2".to_string()),
    price: 679, plu: "3110",
    tags: hashset!("breakfast_combo", "bundle"),
    item_priority: 99,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Items,
            selection: ItemSelection::AnyId(hashset!(steak_junior_breakfast_burrito.id, bacon_junior_breakfast_burrito.id, sausage_junior_breakfast_burrito.id)),
            minimum_quantity: 2,
            maximum_quantity: 2,
            default_quantity: 2,
            price_overrides: vec!(price_override!(item_ids: hashset!(steak_junior_breakfast_burrito.id, bacon_junior_breakfast_burrito.id, sausage_junior_breakfast_burrito.id), price: 0))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _scrambler_burrito_combo = item!(
    id: ID::parse_str("fb689474-4a8c-40f9-baf9-96f1c951d248").unwrap(),
    long_name: "Scrambler Burrito Combo",
    short_name: "B3-SCRM",
    label: Some("#B3".to_string()),
    price: 699, plu: "3120",
    tags: hashset!("breakfast_combo", "bundle"),
    item_priority: 98,
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            selection: ItemSelection::AnyId(hashset!(bacon_scrambler_burrito.id, sausage_scrambler_burrito.id, steak_scrambler_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(
                price_override!(item_ids: hashset!(bacon_scrambler_burrito.id, sausage_scrambler_burrito.id), price: 0),
                price_override!(item_ids: hashset!(steak_scrambler_burrito.id), price: 100)
            )
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _spicy_chorizo_breakfast_burrito_combo = item!(
    id: ID::parse_str("fb88463d-5754-4533-910d-590b6757c5ba").unwrap(),
    long_name: "Spicy Chorizo Burrito Combo",
    short_name: "B4-SPCY",
    label: Some("#B4".to_string()),
    price: 635, plu: "4290",
    tags: hashset!("breakfast_combo", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 97,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: true,
            selection: ItemSelection::Id(spicy_chorizo_breakfast_burrito.id),
            default_item_ids: vec!(spicy_chorizo_breakfast_burrito.id),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(spicy_chorizo_breakfast_burrito.id), price: 0))
            ),
        sides.clone(),
        drinks.clone()
        )
    );

    let _make_it_combo_breakfast = item!(
    id: ID::parse_str("f876aab8-49bb-490c-a958-40a49242e6b4").unwrap(),
    long_name: "Breakfast Combo",
    short_name: "Breakfast Combo",
    price: 432, plu: "4500",
    tags: hashset!("breakfast_combo", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 96,
    slots: vec!(
        sides.clone(),
        drinks.clone()
        )
    );

    let _make_it_combo = item!(
    id: ID::parse_str("fdd3d419-dbcd-45a4-86ee-9e51c33bf51b").unwrap(),
    long_name: "Make It A Combo",
    short_name: "MAKIT-CMB",
    price: 432, plu: "1800",
    tags: hashset!("combos", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 2,
    slots: vec!(
        sides.clone(),
        drinks.clone()
        )
    );

    let _coffee_and_donut = item!(
    id: ID::parse_str("bce1e4fc-2f74-4bf1-a56b-80eb27ae35fc").unwrap(),
    long_name: "Coffee And Donut",
    short_name: "Cfe Dnt",
    price: 319, plu: "4300",
    tags: hashset!("breakfast_combo", "bundle"),
    variations: combo_size_variations.clone(),
    default_variation: combo_size_variations[0].id,
    item_priority: 87,
    slots: vec!(
        slot!(
            name: "Entree",
            slot_type: SlotType::Replace,
            collapsed: true,
            selection: ItemSelection::Id(_mexican_donut_bites.id),
            default_item_ids: vec!(_mexican_donut_bites.id),
            minimum_quantity: 1,
            maximum_quantity: 1,
            default_quantity: 1,
            price_overrides: vec!(price_override!(item_ids: hashset!(_mexican_donut_bites.id), price: 0))
            ),
            slot!(
                name: "Drinks",
                slot_type: SlotType::Replace,
                price_overrides: vec!(
                    price_override!(tags: hashset!("softdrink", "premiumdrink"), variation: Some("Small".into())),
                    price_override!(tags: hashset!("softdrink", "premiumdrink"), variation: Some("Medium".into()), price: 49),
                    price_override!(tags: hashset!("softdrink", "premiumdrink"), variation: Some("Large".into()), price: 99),
                ),
                default_item_ids: vec!(_coffee.id),
                selection: ItemSelection::AnyTag(hashset!("drinks")),
                hidden: ItemSelection::AnyTag(hashset!("premiumdrink")),
                default_quantity: 1,
                minimum_quantity: 1
                )
        )
    );

    // Discounts
    let _10_percent = discount!(name: "10% off", identifier: "1", amount: DiscountAmount::PercentOrder(10), single: true,  incombinable: true);
    let _15_percent = discount!(name: "15% off", identifier: "3", amount: DiscountAmount::PercentOrder(15), single: true,  incombinable: true);
    let _20_percent = discount!(name: "20% off", identifier: "5", amount: DiscountAmount::PercentOrder(20), single: true,  incombinable: true);
    let _50_percent = discount!(name: "50% off", identifier: "7", amount: DiscountAmount::PercentOrder(50), single: true,  incombinable: true, max_amount: 600);
    let _50_percent_employee = discount!(name: "50% Employee", identifier: "9", amount: DiscountAmount::PercentOrder(50), single: true,  incombinable: true);
    let _50_percent_manager_assitant = discount!(name: "50% Assistant Manager", identifier: "11", amount: DiscountAmount::PercentOrder(50), single: true,  incombinable: true);
    let _100_percent_manager = discount!(name: "100% Manager", identifier: "13", amount: DiscountAmount::PercentOrder(100), single: true,  incombinable: true);

    let _two_dollars_survey = discount!(name: "$2 off Survey", identifier: "29200", amount: DiscountAmount::Flat(200), single: true, incombinable: true);

    let _five_dollars_off = discount!(name: "74470 $5 off Purchase", identifier: "74470", amount: DiscountAmount::Flat(500), single: true, incombinable: true);

    let _five_dollars_off_twenty_dollar_purchase = discount!(name: "84706 $5 off $20 Purchase", identifier: "84706", amount: DiscountAmount::Flat(500), single: true, incombinable: true,
    constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 2000,
            maximum_amount: 10000
    })));

    let _five_dollars_off_fried_chicken = discount!(name: "69400 $5 off Fried Chicken", identifier: "69400", amount: DiscountAmount::Flat(500), single: true, incombinable: true,
    constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
        selection: ItemSelection::AnyId(hashset!(fried_chicken_potato_snacker.id, citrus_chipotle_fried_chicken_softshell_taco.id)),
        minimum_quantity: 2,
        maximum_quantity: 150
    })));

    let _one_dollar_ninetynine_cents_off_breakfast_burrito = discount!(name: "71802 $1.99 off Breakfast Burrito", identifier: "71802", amount: DiscountAmount::Flat(199), single: true, incombinable: true,
    constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
        selection: ItemSelection::AnyId(hashset!(bacon_and_egg_burrito.id, sausage_and_egg_burrito.id)),
        minimum_quantity: 1,
        maximum_quantity: 150
    })));

    let _free_small_soft_drink = discount!(name: "72762 Free Small Soft Drink", identifier: "72762", amount: DiscountAmount::Flat(219), single: true, incombinable: true);

    let _two_dollars_6pp = discount!(name: "70402 $2 off Six Pack and a Pound", identifier: "70402", amount: DiscountAmount::Flat(200), single: true, incombinable: false,
    constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
        selection: ItemSelection::AnyId(hashset!(six_pack_and_pound_combo_mix.id, six_pack_and_pound_combo_crispy.id, six_pack_and_pound_combo_soft.id)),
        minimum_quantity: 1,
        maximum_quantity: 150
    })));

    let _two_for_three = discount!( name: "21820 2 Taco Bravo for $3", identifier: "21820", amount: DiscountAmount::Flat(258), single: true, incombinable: true, constraints: vec!(
        OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::Id(beef_taco_bravo.id),
            minimum_quantity: 2,
            maximum_quantity: 150
        })
    ));

    let _discount_02 = discount!(name: "620401 Breakfast Grillers/Cold Brew: $5 off", identifier: "62401", amount: DiscountAmount::Flat(500), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(potato_griller_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2021-01-28, end:2021-08-01

    let _discount_07 = discount!(name: "74350 $2 Off Beef or Chicken Taco Salad", identifier: "74350", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_taco_salad.id, chicken_taco_salad.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-01-27, end:2021-08-29

        let _discount_24303 = discount!(name: "24303 $2 Off Taco Salad", identifier: "24303", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(taco_salad_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-01-27, end:2021-08-29
    
    let _discount_12 = discount!(name: "70502 Free Small Potato Oles", identifier: "70502", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(potato_ole_side.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-01-27, end:2021-08-29

    // let _discount_13 = discount!(name: "$1 Off Any Combo Meal", identifier: "71100", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2018-03-01, end:2021-12-31

    let _discount_16 = discount!(name: "80602 $2 Off Taco Salad beef only", identifier: "80602", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_taco_salad.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-04-20, end:2021-12-31

    let _discount_23 = discount!(name: "73361 $2 Off Boss Burrito or Bowl", identifier: "73361", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(boss_burrito_shell_item.id, boss_bowl_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-04-20, end:2021-12-31

    // let _discount_24 = discount!(name: "81400 $1 Off Any lunch Combo (before 10AM)", identifier: "81400", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2016-08-22, end:2021-12-31

    let _discount_25 = discount!(name: "84300 $2 Off Any Super Nacho Beef Or Chicken Only", identifier: "84300", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_super_nachos.id, chicken_super_nachos.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-07-02, end:2021-12-31

    let _discount_29 = discount!(name: "85700 Free Small Potato Oles", identifier: "85700", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(potato_ole_side.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_30 = discount!(name: "87500 $2 Off SPAAP", identifier: "87500", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_31 = discount!(name: "89700 Free Crispy Beef Taco With Any Purchase", identifier: "89700", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_32 = discount!(name: "89701 Free Beef Softshell Taco", identifier: "89701", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_softshell_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2018-06-28, end:2021-12-31

    let _discount_33 = discount!(name: "89711 Free Taco", identifier: "89711", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(taco_shell_item.id, softshell_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2018-06-28, end:2021-12-31

    let _discount_34 = discount!(name: "89712 (1) Free Fried Chicken Taco", identifier: "89712", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(fried_chicken_taco_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_35 = discount!(name: "80500 $2 Off Next Purchase", identifier: "80500", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 200,
            maximum_amount: 10000
    }))); //start: 2016-08-22, end:2021-12-31

    let _discount_36 = discount!(name: "81100 $1 Off Any M&P Breakfast Burrito", identifier: "81100", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(meat_and_potato_breakfast_burrito_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-08-01, end:2021-12-31

    let _discount_38 = discount!(name: "81700 Free Meat & Potato Burrito", identifier: "81700", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(meat_and_potato_burrito_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-07-10, end:2021-12-31

    let _discount_39 = discount!(name: "84101 $1 Off Super Nachos on Next Visit", identifier: "84101", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(super_nachos_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_40 = discount!(name: "84701 Free Kids Meal", identifier: "84701", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(_kids_meal.id, _kids_meal_soft.id, _kids_meal_burr.id, _kids_meal_taco_burger.id, _kids_meal_fried_chicken.id, _kids_meal_quesadilla.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    // let _discount_41 = discount!(name: "84702 Free Combo Meal", identifier: "84702", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2019-08-01, end:2021-12-31

    // let _discount_42 = discount!(name: "86700 Free Churro", identifier: "86700", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2019-08-01, end:2021-12-31

    let _discount_45 = discount!(name: "89710 Free Crispy Beef Taco for Reading", identifier: "89710", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2016-08-22, end:2021-12-31

    let _discount_46 = discount!(name: "29200 $2 Off Purchase", identifier: "29200", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 200,
            maximum_amount: 10000
    }))); //start: 2021-02-01, end:2021-12-31

    let _discount_47 = discount!(name: "74381 $2 Off Taco Salad", identifier: "74381", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(taco_salad_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-08-01, end:2021-12-31

    let _discount_48 = discount!(name: "74780 $2 Off SPAAP", identifier: "74780", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2017-06-15, end:2021-12-31

    let _discount_25301 = discount!(name: "25301 $2 Off SPAAP", identifier: "25301", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2017-06-15, end:2021-12-31

    let _discount_50 = discount!(name: "77781 Free Kids Meal", identifier: "77781", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(_kids_meal.id, _kids_meal_soft.id, _kids_meal_burr.id, _kids_meal_taco_burger.id, _kids_meal_fried_chicken.id, _kids_meal_quesadilla.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-08-01, end:2021-12-31

    // let _discount_51 = discount!(name: "74351 $2 Off Any Combo", identifier: "74351", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2020-06-13, end:2021-12-31

    let _discount_52 = discount!(name: "74401 Taco Bucks: $5 Off Purchase", identifier: "74401", amount: DiscountAmount::Flat(500), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 500,
            maximum_amount: 10000
    }))); //start: 2017-05-15, end:2021-12-31

    let _discount_53 = discount!(name: "74460 $2 Off Purchase", identifier: "74460", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 200,
            maximum_amount: 10000
    }))); //start: 2021-04-09, end:2021-12-31

    let _discount_55 = discount!(name: "74530 $10 Off Purchase", identifier: "74530", amount: DiscountAmount::Flat(1000), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::OrderTotal(OrderTotalConstraint{
            minimum_amount: 1000,
            maximum_amount: 10000
    }))); //start: 2020-06-08, end:2021-12-31

    let _discount_57 = discount!(name: "75792 Free Small Potato Oles", identifier: "75792", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(potato_ole_side.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2017-05-15, end:2021-12-31

    let _discount_58 = discount!(name: "76793 Free Dessert", identifier: "76793", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(_cinnamon_sugar_tortilla_crisps.id, _mexican_donut_bites.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2021-04-09, end:2021-12-31

    /*let _discount_59 = discount!(name: "76794 Free Churro bites with any purchase", identifier: "76794", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!()),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-09-13, end:2021-12-31*/

    let _discount_60 = discount!(name: "76795 Free Donut bites", identifier: "76795", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(_mexican_donut_bites.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-06-08, end:2021-12-31

    //how to specify drink constraint?
    let _discount_62 = discount!(name: "77752 Free Crispy Beef Taco, Small Potato Oles, Small Soft Drink (Medical Professionals, 1st Responders)", identifier: "77752", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, potato_ole_side.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2017-09-10, end:2021-12-31

    let _discount_63 = discount!(name: "77770 Free Kids Meal", identifier: "77770", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(_kids_meal.id, _kids_meal_soft.id, _kids_meal_burr.id, _kids_meal_taco_burger.id, _kids_meal_fried_chicken.id, _kids_meal_quesadilla.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2017-08-15, end:2021-12-31

    let _discount_64 = discount!(name: "77780 2 Free Bean BurritoS With SPAAP", identifier: "77780", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id, bean_burrito.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-07-16, end:2021-12-31

    // let _discount_65 = discount!(name: "77790 Free Any Combo Meal", identifier: "77790", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2021-04-09, end:2021-12-31

    let _discount_68 = discount!(name: "78770 Free Small Potato Oles", identifier: "78770", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(potato_ole_side.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-09-13, end:2021-12-31

    let _discount_69 = discount!(name: "79250 $1 Off SPAAP At Regular Price", identifier: "79250", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-09-13, end:2021-12-31

    let _discount_70 = discount!(name: "79372 $2 Off SPAAP  (Limit 4)", identifier: "79372", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 4
        }))); //start: 2020-05-18, end:2021-12-31

    let _discount_72 = discount!(name: "79755 Free SPAAP", identifier: "79755", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2020-11-03, end:2021-12-31

    // let _discount_73 = discount!(name: "79790 Free Taco with any Combo Purchase", identifier: "79790", amount: DiscountAmount::Flat(100), single: true, incombinable: true,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!()),
    //         minimum_quantity: 1,
    //         maximum_quantity: 150
    //     }))); //start: 2019-09-13, end:2021-12-31
        
    let _discount_80 = discount!(name: "79370 $2 Off SPAAP", identifier: "79370", amount: DiscountAmount::Flat(200), single: true, incombinable: true,
        constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
            selection: ItemSelection::AnyId(hashset!(six_pack_shell_item.id)),
            minimum_quantity: 1,
            maximum_quantity: 150
        }))); //start: 2019-09-13, end:2022-02-28

    // let _discount_25900 = discount!(name: "2 for $5", identifier: "25900", amount: DiscountAmount::Set(500), single: true, incombinable: false,
    //     constraints: vec!(OrderConstraint::ItemQuantity(ItemQuantityConstraint {
    //         selection: ItemSelection::AnyId(hashset!(beef_stuffed_grilled_taco.id, stuffed_grilled_chipotle_chicken_taco.id, steak_stuffed_grilled_taco.id, bean_stuffed_grilled_taco.id,stuffed_grilled_chipotle_chicken_taco.id)),
    //         minimum_quantity: 2,
    //         maximum_quantity: 150
    //     }))); //start: 2017-06-15, end:2021-12-31


    // Dynamic Pricing
    let taco_price_mod = pricing_modification!(style: PricingModificationStyle::Flat, amount: -70);
    let bean_burr_price_mod = pricing_modification!(style: PricingModificationStyle::Flat, amount: -40);
    let taco_tuesday_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(beef_crispy_taco.id, beef_softshell_taco.id)),
        pricing_modification: taco_price_mod
    );
    let bean_burr_pricing_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(bean_burrito.id)),
        pricing_modification: bean_burr_price_mod
    );
    dynamic_price!(auto_constraints: OrderConstraint::Time(OrderTimeConstraint {
        day_of_week: vec![chrono::Weekday::Tue],
        start_time: 0,
        stop_time: 86400,
    }), rules: vec![taco_tuesday_price_rule, bean_burr_pricing_rule], name: "Taco Tuesday");

    let wake_up_wed_mod = pricing_modification!(style: PricingModificationStyle::Set, amount: 249);
    let wake_up_wed_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(beef_potato_breakfast_burrito.id, bacon_potato_breakfast_burrito.id, sausage_potato_breakfast_burrito.id, saus_and_bacon_meat_potato_burrito.id, bacon_scrambler_burrito.id, sausage_scrambler_burrito.id, bacon_and_egg_burrito.id, sausage_and_egg_burrito.id, triple_meat_potato_breakfast_burrito.id)),
        pricing_modification: wake_up_wed_mod
    );
    let wake_up_wed_steak_mod = pricing_modification!(style: PricingModificationStyle::Set, amount: 309);
    let wake_up_wed_steak_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(steak_potato_breakfast_burrito.id, steak_scrambler_burrito.id, steak_and_egg_burrito.id)),
        pricing_modification: wake_up_wed_steak_mod
    );
    dynamic_price!(auto_constraints: OrderConstraint::Time(OrderTimeConstraint {
        day_of_week: vec![chrono::Weekday::Wed],
        start_time: 0,
        stop_time: 86400,
    }), rules: vec![wake_up_wed_price_rule, wake_up_wed_steak_price_rule], name: "Wake up Wednesday");

    let taco_bravo_price_mod = pricing_modification!(style: PricingModificationStyle::Flat, amount: -120);
    let taco_bravo_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(bean_taco_bravo.id, beef_taco_bravo.id, chicken_taco_bravo.id, steak_taco_bravo.id)),
        pricing_modification: taco_bravo_price_mod
    );
    dynamic_price!(auto_constraints: OrderConstraint::Time(OrderTimeConstraint {
        day_of_week: vec![chrono::Weekday::Thu],
        start_time: 0,
        stop_time: 86400,
    }), rules: vec![taco_bravo_price_rule], name: "Taco Bravo Thursday");

    let six_pack_sunday = pricing_modification!(style: PricingModificationStyle::Flat, amount: -150);
    let six_pack_sunday_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(six_pack_and_pound_combo_crispy.id, six_pack_and_pound_combo_mix.id, six_pack_and_pound_combo_soft.id)),
        pricing_modification: six_pack_sunday
    );
    dynamic_price!(auto_constraints: OrderConstraint::Time(OrderTimeConstraint {
        day_of_week: vec![chrono::Weekday::Sun],
        start_time: 0,
        stop_time: 86400,
    }), rules: vec![six_pack_sunday_price_rule], name: "Six Pack Sunday");

    let taco_salad_monday = pricing_modification!(style: PricingModificationStyle::Flat, amount: -100);
    let taco_salad_monday_price_rule = pricing_rule!(
        selection: ItemSelection::AnyId(hashset!(beef_taco_salad.id, chicken_taco_salad.id, steak_taco_salad.id)),
        pricing_modification: taco_salad_monday
    );
    dynamic_price!(auto_constraints: OrderConstraint::Time(OrderTimeConstraint {
        day_of_week: vec![chrono::Weekday::Mon],
        start_time: 0,
        stop_time: 86400,
    }), rules: vec![taco_salad_monday_price_rule], name: "Taco Salad Monday");

    let json = match serde_json::to_string_pretty(&menu) {
        Ok(json) => json,
        Err(e) => panic!("{}", e),
    };

    let mut file = File::create("menu.json").expect("Couldnt create file");
    file.write_all(json.as_bytes())
        .expect("Couldnt write menu file");
}
