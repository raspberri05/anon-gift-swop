use js_sys::Array;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::forbidden_pair::ForbiddenPair;
use assignment::{Assignment, AssignmentPair};
use default_map::DefaultMap;
use threshold_selector::ThresholdSelector;
use total_weight::TotalWeight;
use walker::SolutionWalker;

mod default_map;
mod assignment;
mod forbidden_pair;
mod walker;
mod total_weight;
mod threshold_selector;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn process_assignment(
    selection: JsValue,
    forbidden_pairs: JsValue,
    history: JsValue,
) -> JsValue {
    let selection_set = to_rust_selection(selection);
    let forbidden_pairs = to_rust_forbidden_pairs(forbidden_pairs);
    let history_vec = to_rust_history(history);

    let mut buffer: Vec<_> = selection_set.iter().cloned().collect();

    let weights = weights_for(&selection_set, &forbidden_pairs, &history_vec);

    // Get the total weight of all possible valid selections
    let mut total_weight_walker = TotalWeight::default();
    walker::apply_walker(&weights, &mut buffer, &mut total_weight_walker);
    let total_weight = total_weight_walker.result();

    // Randomly select a given threshold for selection
    let r = random();

    let weight_threshold: f64 = r *total_weight;

    // Walk again to extract the selection
    let mut threshold_selector_walker = ThresholdSelector::new(weight_threshold);
    walker::apply_walker(&weights, &mut buffer, &mut threshold_selector_walker);
    let selection = threshold_selector_walker.result();


    match selection {
        None => JsValue::UNDEFINED,
        Some(selection) => JsValue::from(selection.iter().map(|&number| JsValue::from_f64(number as f64)).collect::<Array>()),
    }
}


fn weights_for(participants: &HashSet<usize>, forbidden_pairs: &Vec<ForbiddenPair>, previous_assignments: &Vec<Assignment>) -> DefaultMap<AssignmentPair, f64> {
    // useful set for filtering out non-participants
    let participant_set: HashSet<_> = participants.iter().cloned().collect();

    // Default weight will be 1.0
    // 0.0 will be an impossible assignment
    // Values between are due to disincentive based on previous assignments
    let default_weight = 1.0;
    let mut weights = HashMap::new();

    // forbidden pairs
    for fp in forbidden_pairs
    {
        // ignore entries which are not part of the current set of participants
        if fp.invalid(&participant_set) { continue; }

        // both giving and receiving assignments are invalid
        weights.insert(AssignmentPair { giver: fp.member_1, receiver: fp.member_2 }, 0.0);
        weights.insert(AssignmentPair { giver: fp.member_2, receiver: fp.member_1 }, 0.0);
    }

    // disincentive for recent history
    for &giver in participants {
        let disincentives = (0..).map(|i| 0.5 / 2.0_f64.powi(i));
        let receiver_penalties = previous_assignments.iter()
            // selection that the `giver` was involved in, get the receiver
            .filter_map(|selection| selection.receiver_of(giver))
            // check receiver is relevant to the current participants
            .filter(|receiver| participant_set.contains(receiver))
            // historic assignment have decreasing disincentives
            .zip(disincentives);

        for (receiver, penalty) in receiver_penalties {
            weights.entry(AssignmentPair { giver, receiver })
                .and_modify(|w| *w -= penalty)
                .or_insert(default_weight - penalty);
        }
    }

    DefaultMap { map: weights, default_value: default_weight }
}

fn to_rust_history(history: JsValue) -> Vec<Assignment> {
    history
        .dyn_ref::<Array>()
        .unwrap()
        .to_vec()
        .iter()
        .filter_map(|v| {
            let arr = v.dyn_ref::<Array>()?;
            let value: Vec<_> = arr
                .to_vec()
                .iter()
                .filter_map(|v| v.as_f64())
                .map(|v| v as usize)
                .collect();
            Some(Assignment { value })
        })
        .collect()
}

fn to_rust_forbidden_pairs(forbidden_pairs: JsValue) -> Vec<ForbiddenPair> {
    forbidden_pairs
        .dyn_ref::<Array>()
        .unwrap()
        .to_vec()
        .iter()
        .filter_map(|v| {
            let arr = v.dyn_ref::<Array>()?;
            let first = arr.get(0).as_f64()? as usize;
            let second = arr.get(1).as_f64()? as usize;
            if first < second {
                Some(ForbiddenPair { member_1: first, member_2: second })
            } else {
                Some(ForbiddenPair { member_1: second, member_2: first })
            }
        })
        .collect()
}

fn to_rust_selection(selection: JsValue) -> HashSet<usize> {
    selection
        .dyn_ref::<Array>()
        .unwrap()
        .to_vec()
        .iter()
        .filter_map(|v| v.as_f64())
        .map(|v| v as usize)
        .collect()
}