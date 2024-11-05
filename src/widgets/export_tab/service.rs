// Copyright 2024 the Cartero authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use askama::Template;

use crate::client::BoundRequest;
use crate::entities::EndpointData;
use crate::error::CarteroError;

mod templates {
    use crate::client::BoundRequest;
    use askama::Template;
    use serde_json::Value;
    use std::convert::From;

    #[macro_export]
    macro_rules! generate_template_struct {
        ($struct_name:ident, $template_path:expr) => {
            #[derive(Template)]
            #[template(path = $template_path)]
            pub struct $struct_name {
                pub url: String,
                pub method: String,
                pub headers: std::collections::HashMap<String, String>,
                pub body: Option<String>,
            }

            impl From<BoundRequest> for $struct_name {
                fn from(value: BoundRequest) -> Self {
                    Self {
                        url: value.url,
                        method: value.method.into(),
                        headers: value.headers,
                        body: value.body.map(|v| {
                            let body = String::from_utf8_lossy(&v).to_string();

                            serde_json::from_str(body.as_ref()).map_or(body, |v: Value| {
                                serde_json::to_string(&v).unwrap().replace("'", "\\\\'")
                            })
                        }),
                    }
                }
            }
        };
    }

    generate_template_struct!(CurlTemplate, "curl");
}

pub struct CodeExportService {
    endpoint_data: EndpointData,
}

impl CodeExportService {
    pub fn new(endpoint_data: EndpointData) -> Self {
        Self { endpoint_data }
    }

    pub fn into_curl_like(&self) -> Result<String, CarteroError> {
        let bound_request = BoundRequest::try_from(self.endpoint_data.clone())?;
        let template: templates::CurlTemplate = bound_request.into();

        template.render().map_err(|_| CarteroError::AskamaFailed)
    }
}
