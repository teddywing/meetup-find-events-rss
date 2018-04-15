// Copyright © 2018 Teddy Wing
//
// This file is part of Meetup Find Events RSS.
//
// Meetup Find Events RSS is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// Meetup Find Events RSS is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Meetup Find Events RSS. If not, see
// <http://www.gnu.org/licenses/>.

#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub link: String,
    pub local_date: Option<String>,
    pub local_time: Option<String>,
    pub venue: Option<Venue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Venue {
    pub name: String,
    pub address_1: String,
    pub city: String,
    pub localized_country_name: String,
}
