/// Contains mapping functions that are related to SNOW (ServiceNow) data structures
use crate::servicenow::AssignmentGroup;
use std::collections::HashMap;

/// Creates a map for the internal SNOW assigment group id to an ID that is more suitable for fine tuning the OpenAI model
/// (see: <https://beta.openai.com/docs/guides/fine-tuning/case-study-categorization-for-email-triage>).
///
/// To make sure that the generated ID does not change for existing assignment groups, when new ones are created, we sort
/// by creation date of the assignment groups first, before we take the index of the position within the list, as ID.
///
/// # Arguments
///
/// - `assignment_groups`: The raw parsed assignment groups from the SNOW export
///
/// # Returns
///
/// Map with assignment group ID as key and mapped ID which can be used for categorization with OpenAI
pub fn map_assignment_groups(
    assignment_groups: &mut Vec<AssignmentGroup>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (idx, assignment_group) in order_assignment_groups(assignment_groups)
        .iter()
        .enumerate()
    {
        result.insert(assignment_group.sys_id.clone(), idx);
    }
    result
}

/// Sorts the assignment groups ascending by the creation date of the assignment group
///
/// # Arguments
///
/// - `assignment_groups`: List of unsorted assignment groups
///
/// # Returns
///
/// The sorted assignment groups
fn order_assignment_groups(assignment_groups: &mut Vec<AssignmentGroup>) -> &Vec<AssignmentGroup> {
    assignment_groups.sort_by(|a, b| a.sys_created_on.partial_cmp(&b.sys_created_on).unwrap());
    assignment_groups
}
