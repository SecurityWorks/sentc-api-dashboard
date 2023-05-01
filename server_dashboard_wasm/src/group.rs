use alloc::string::String;
use alloc::vec::Vec;

use sentc_crypto::util::public::{handle_general_server_response, handle_server_response};
use sentc_crypto_full::util::{make_req, HttpMethod};
use serde::Serialize;
use server_api_common::customer::{CustomerGroupCreateInput, CustomerGroupList, CustomerGroupMemberFetch, CustomerGroupView};
use server_api_common::sdk_common::group::{GroupChangeRankServerInput, GroupCreateOutput, GroupNewMemberLightInput};

use crate::utils;

pub async fn create_group(base_url: String, jwt: &str, name: Option<String>, des: Option<String>) -> Result<String, String>
{
	let input = CustomerGroupCreateInput {
		name,
		des,
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/group";

	let res = make_req(HttpMethod::POST, &url, "", Some(input), Some(jwt), None).await?;
	let out: GroupCreateOutput = handle_server_response(&res)?;

	Ok(out.group_id)
}

pub async fn get_all_groups(base_url: String, jwt: &str, last_fetched_time: &str, last_id: &str) -> Result<Vec<CustomerGroupList>, String>
{
	let url = base_url + "/api/v1/customer/group/all/" + last_fetched_time + "/" + last_id;

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt), None).await?;

	let out: Vec<CustomerGroupList> = handle_server_response(&res)?;

	Ok(out)
}

pub async fn get_group(base_url: String, jwt: &str, group_id: &str) -> Result<CustomerGroupView, String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id;

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt), None).await?;

	let out: CustomerGroupView = handle_server_response(&res)?;

	Ok(out)
}

pub async fn invite_member(base_url: String, jwt: &str, group_id: &str, user_id: &str, rank: Option<i32>) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id + "/invite/" + user_id;

	let input = GroupNewMemberLightInput {
		rank,
	};
	let input = utils::to_string(&input)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}

#[derive(Serialize)]
pub struct CustomerGroupMemberListItem
{
	pub user_id: String,
	pub rank: i32,
	pub joined_time: u128,
	pub first_name: String,
	pub name: String,
	pub email: String,
}

/**
Merge both lists together.

The server just returned it as two lists because there are two db fetches.
*/
pub async fn get_member_list(
	base_url: String,
	jwt: &str,
	group_id: &str,
	last_fetched_time: &str,
	last_id: &str,
) -> Result<Vec<CustomerGroupMemberListItem>, String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id + "/member/" + last_fetched_time + "/" + last_id;

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt), None).await?;

	let out: CustomerGroupMemberFetch = handle_server_response(&res)?;

	if out.group_member.is_empty() {
		return Ok(Vec::new());
	}

	let len = out.group_member.len();
	let mut list = Vec::with_capacity(out.group_member.len());

	let mut group_user = out.group_member.into_iter();
	let mut user_info = out.customer_data.into_iter();

	for i in 0..len {
		let group_member_info = group_user.nth(i);

		let user_info = user_info.nth(i);

		match (group_member_info, user_info) {
			(Some(g), Some(u)) => {
				list.push(CustomerGroupMemberListItem {
					user_id: g.user_id,
					rank: g.rank,
					joined_time: g.joined_time,
					first_name: u.first_name,
					name: u.name,
					email: u.email,
				})
			},
			_ => continue,
		}
	}

	Ok(list)
}

pub async fn update_user_rank(base_url: String, jwt: &str, group_id: &str, user_id: String, new_rank: i32) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id + "/change_rank";

	let input = GroupChangeRankServerInput {
		changed_user_id: user_id,
		new_rank,
	};
	let input = utils::to_string(&input)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}

pub async fn kick_user(base_url: String, jwt: &str, group_id: &str, user_id: &str) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id + "/kick/" + user_id;

	let res = make_req(HttpMethod::DELETE, url.as_str(), "", None, Some(jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}

pub async fn update_group(base_url: String, jwt: &str, group_id: &str, name: Option<String>, des: Option<String>) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id + "/update";

	let input = CustomerGroupCreateInput {
		name,
		des,
	};
	let input = utils::to_string(&input)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}

pub async fn delete_group(base_url: String, jwt: &str, group_id: &str) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/group/" + group_id;

	let res = make_req(HttpMethod::DELETE, url.as_str(), "", None, Some(jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}
