// use chrono::ParseResult;
use mysql::prelude::*;
use mysql::*;
// use chrono::{Local, DateTime};

use std::time::SystemTime;

#[derive(Debug, PartialEq, Eq)]
struct Employee {
    employee_id: i64,
    employee_fname: String,
    employee_lname: String,
    employee_mail: String,
    password: String,
    user_type: i64,
}

fn main() {
    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let now = SystemTime::now();

    //retrieving data for login
    fn login(cn: &mut PooledConn, pasw: String, empid: i64) {
        let y=format!("select EmployeeID, EmployeeFirstName, EmployeeLastName, EmployeeEmail, password, UserTypeId from employee where EmployeeID= {}",empid);
        let res = cn
            .query_map(
                y,
                |(
                    employee_id,
                    employee_fname,
                    employee_lname,
                    employee_mail,
                    password,
                    user_type,
                )| Employee {
                    employee_id: employee_id,
                    employee_fname: employee_fname,
                    employee_lname: employee_lname,
                    employee_mail: employee_mail,
                    password: password,
                    user_type: user_type,
                },
            )
            .expect("Query failed.");
        let mut pass: String = String::new();
        let mut mail: String = String::new();
        let mut esid = 0;
        let mut name: String = String::new();
        let mut utid: i64 = 0;
        for r in res {
            pass = r.password;
            mail = r.employee_mail;
            esid = r.employee_id;
            name = r.employee_fname;
            utid = r.user_type;
        }


        if pasw.trim() == pass {
            println!("Welcome {}", name);
        } else {
            println!("Login failed");
        }
    }

    let mut username = String::new();
    println!("Enter your employee id :");
    std::io::stdin().read_line(&mut username).unwrap();
    let empid: i64 = username.trim().parse().expect("username");
    println!("Employee ID:{}", empid);

    let mut pswrd = String::new();
    println!("Enter your password :");
    std::io::stdin().read_line(&mut pswrd).unwrap();

    login(&mut conn, pswrd, empid);
}
