use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use ynab_api::models;

use crate::args::*;
use crate::constants::*;
use crate::output::*;
use crate::types::*;
use crate::ynab_state::*;

pub fn list_categories(state: &YnabState) -> Result<(), AnyError> {
    // let include_hidden: bool = req_parse_value_of(state.matches, INCLUDE_HIDDEN_ARG);
    let opt_month = opt_month_value_of(state.matches, MONTH_ARG);
    if let Some(month) = opt_month {
        let response = state.run(&|c| {
            c.months_api()
                .get_budget_month(&state.global.budget_id, &month)
        })?;
        htable_output(
            state,
            response.data(),
            response.data().month().categories(),
            &make_categories_table,
        )
    } else {
        let response =
            state.run(&|c| c.categories_api().get_categories(&state.global.budget_id))?;
        // let mut groups: Vec<models::CategoryGroupWithCategories> = Vec::new();
        // for group in response.data().category_groups() {
        //     if include_hidden || !group.hidden() {
        //         let mut group = group.to_owned();
        //         let mut cats = Vec::new();
        //         for cat in group.categories() {
        //             if include_hidden || !cat.hidden() {
        //                 cats.push(cat.to_owned());
        //             }
        //         }
        //         group.set_categories(cats);
        //         groups.push(group);
        //     }
        // }
        // htable_output(state, &groups, &make_category_groups_table)
        htable_output(
            state,
            response.data(),
            response.data().category_groups(),
            &make_category_groups_table,
        )
    }
}

pub fn get_category(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let month = req_month_value_of(state.matches, MONTH_ARG);
    // @@@ CURRENTY API'S BY-MONTH SEEMS TO IGNORE THE MONTH AND ASSUME CURRENT
    let response = state.run(&|c| {
        c.categories_api()
            .get_month_category_by_id(&state.global.budget_id, &month, id)
    })?;
    vtable_output(
        state,
        response.data(),
        response.data().category(),
        &make_category_table,
    )
}

pub fn update_category(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let month = req_month_value_of(state.matches, MONTH_ARG);
    let settings = state.get_budget_settings()?;
    let model = file_input(state)?.unwrap_or({
        let budgeted = req_milliunits_value_of(&settings, state.matches, SET_BUDGETED_ARG)?;
        models::SaveMonthCategoryWrapper::new(models::SaveMonthCategory::new(budgeted))
    });
    let response = state.run(&|c| {
        c.categories_api().update_month_category(
            &state.global.budget_id,
            &month,
            id,
            model.clone(), // @@@ NOT SURE WHY WE NEED TO CLONE HERE
        )
    })?;
    // @@@ PRINT "UPDATED" MESSAGE (WITH LOGGING INFO LEVEL), HERE AND ELSEWHERE
    vtable_output(
        state,
        response.data(),
        response.data().category(),
        &make_category_table,
    )
}

pub fn category_cell(
    settings: &models::BudgetSettings,
    group: Option<&models::CategoryGroupWithCategories>,
    cat: &models::Category,
    col: &CategoryCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        CategoryCol::GroupId => Cell::new(&opt_ref_str(group.map(|g| g.id()))),
        CategoryCol::GroupName => Cell::new(&opt_ref_str(group.map(|g| g.name()))),
        CategoryCol::GroupHidden => Cell::new(&opt_to_str(group.map(|g| g.hidden()))),
        CategoryCol::GroupDeleted => Cell::new(&opt_to_str(group.map(|g| g.deleted()))),
        CategoryCol::Id => Cell::new(cat.id()),
        CategoryCol::Name => Cell::new(cat.name()),
        CategoryCol::Hidden => Cell::new(&cat.hidden().to_string()),
        CategoryCol::OriginalCategoryGroupId => {
            Cell::new(&opt_ref_str(cat.original_category_group_id()))
        }
        CategoryCol::Note => Cell::new(&opt_ref_str(cat.note())),
        CategoryCol::Budgeted => Cell::new_align(
            &milliunits_str(settings, cat.budgeted()),
            currency_alignment,
        ),
        CategoryCol::Activity => Cell::new_align(
            &milliunits_str(settings, cat.activity()),
            currency_alignment,
        ),
        CategoryCol::Balance => {
            Cell::new_align(&milliunits_str(settings, cat.balance()), currency_alignment)
        }
        CategoryCol::GoalType => Cell::new(&opt_ref_str(cat.goal_type())),
        CategoryCol::GoalCreationMonth => Cell::new(&opt_month_str(cat.goal_creation_month())),
        CategoryCol::GoalTarget => Cell::new_align(
            &milliunits_str(settings, cat.goal_target()),
            currency_alignment,
        ),
        CategoryCol::GoalTargetMonth => Cell::new(&opt_month_str(cat.goal_target_month())),
        CategoryCol::GoalPercentageComplete => {
            Cell::new(&cat.goal_percentage_complete().to_string())
        }
        CategoryCol::Deleted => Cell::new(&cat.deleted().to_string()),
    }
}

fn category_col_alignments(col: &CategoryCol) -> Alignment {
    match *col {
        CategoryCol::Budgeted => Alignment::RIGHT,
        CategoryCol::Activity => Alignment::RIGHT,
        CategoryCol::Balance => Alignment::RIGHT,
        CategoryCol::GoalTarget => Alignment::RIGHT,
        _ => Alignment::LEFT,
    }
}

fn make_category_groups_table(
    state: &YnabState,
    columns: &Vec<CategoryCol>,
    groups: &Vec<models::CategoryGroupWithCategories>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut table = make_htable(state, columns, &category_col_alignments);
    for group in groups {
        for cat in group.categories() {
            table.add_row(Row::new(
                columns
                    .iter()
                    .map(|c| category_cell(&settings, Some(group), cat, c, Alignment::RIGHT))
                    .collect(),
            ));
        }
    }
    Ok(table)
}

fn make_categories_table(
    state: &YnabState,
    columns: &Vec<CategoryCol>,
    categories: &Vec<models::Category>,
) -> Result<Table, AnyError> {
    // @@@ TODO: LOOKUP GROUPS
    let settings = state.get_budget_settings()?;
    let mut table = make_htable(state, columns, &category_col_alignments);
    for cat in categories {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|c| category_cell(&settings, None, cat, c, Alignment::RIGHT))
                .collect(),
        ));
    }
    Ok(table)
}

fn make_category_table(state: &YnabState, category: &models::Category) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut table = make_vtable(state);
    // @@@ TODO: LOOKUP THE GROUP AND SHOW IT TOO
    for col in CATEGORY_NON_GROUP_COLS.iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            category_cell(&settings, None, category, col, Alignment::LEFT),
        ]));
    }
    Ok(table)
}
