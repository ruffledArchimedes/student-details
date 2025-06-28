// use candid::{CandidType, Deserialize};
// use ic_cdk::api::time;
// use ic_cdk_macros::*;
// use std::cell::RefCell;
// use std::collections::HashMap;

// #[derive(CandidType, Deserialize, Clone, Debug)]
// pub struct Student {
//     pub name: String,
//     pub total_marks: f64,
//     pub num_subjects: u32,
//     pub average: f64,
//     pub grade: String,
//     pub timestamp: u64,
// }

// #[derive(CandidType, Deserialize)]
// pub struct StudentInput {
//     pub name: String,
//     pub total_marks: f64,
//     pub num_subjects: u32,
// }

// #[derive(CandidType, Deserialize)]
// pub struct ReportCard {
//     pub student: Student,
//     pub report_html: String,
// }

// thread_local! {
//     static STUDENTS: RefCell<HashMap<String, Student>> = RefCell::new(HashMap::new());
//     static STUDENT_COUNTER: RefCell<u64> = RefCell::new(0);
// }

// impl Student {
//     fn new(name: String, total_marks: f64, num_subjects: u32) -> Self {
//         let average = calculate_average(total_marks, num_subjects);
//         let grade = assign_grade(average);
        
//         Student {
//             name,
//             total_marks,
//             num_subjects,
//             average,
//             grade,
//             timestamp: time(),
//         }
//     }

//     fn to_report_html(&self) -> String {
//         format!(
//             r#"
//             <!DOCTYPE html>
//             <html>
//             <head>
//                 <title>Report Card - {}</title>
//                 <style>
//                     body {{ font-family: Arial, sans-serif; margin: 20px; }}
//                     .report-card {{ 
//                         border: 3px solid #333; 
//                         padding: 20px; 
//                         max-width: 500px; 
//                         margin: 0 auto;
//                         background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
//                         color: white;
//                         border-radius: 10px;
//                     }}
//                     .header {{ text-align: center; font-size: 24px; font-weight: bold; margin-bottom: 20px; }}
//                     .field {{ margin: 10px 0; font-size: 18px; }}
//                     .grade {{ font-size: 24px; font-weight: bold; text-align: center; margin: 20px 0; }}
//                     .grade-scale {{ margin-top: 30px; font-size: 14px; }}
//                 </style>
//             </head>
//             <body>
//                 <div class="report-card">
//                     <div class="header">🎓 STUDENT REPORT CARD</div>
//                     <div class="field"><strong>Student Name:</strong> {}</div>
//                     <div class="field"><strong>Total Marks:</strong> {}</div>
//                     <div class="field"><strong>Number of Subjects:</strong> {}</div>
//                     <div class="field"><strong>Average:</strong> {:.2}</div>
//                     <div class="grade">Grade: {}</div>
//                     <div class="grade-scale">
//                         <strong>Grade Scale:</strong><br>
//                         A: 90+ (Excellent)<br>
//                         B: 75-89 (Good)<br>
//                         C: 60-74 (Average)<br>
//                         D: Below 60 (Needs Improvement)
//                     </div>
//                 </div>
//             </body>
//             </html>
//             "#,
//             self.name, self.name, self.total_marks, self.num_subjects, self.average, self.grade
//         )
//     }
// }

// // Custom function to calculate average
// fn calculate_average(total_marks: f64, num_subjects: u32) -> f64 {
//     if num_subjects == 0 {
//         return 0.0;
//     }
//     total_marks / num_subjects as f64
// }

// // Function to assign grade based on average
// fn assign_grade(average: f64) -> String {
//     match average {
//         avg if avg >= 90.0 => "A".to_string(),
//         avg if avg >= 75.0 => "B".to_string(),
//         avg if avg >= 60.0 => "C".to_string(),
//         _ => "D".to_string(),
//     }
// }

// #[update]
// pub fn add_student(input: StudentInput) -> Result<Student, String> {
//     if input.name.trim().is_empty() {
//         return Err("Student name cannot be empty".to_string());
//     }
    
//     if input.total_marks < 0.0 {
//         return Err("Total marks cannot be negative".to_string());
//     }
    
//     if input.num_subjects == 0 {
//         return Err("Number of subjects must be greater than 0".to_string());
//     }

//     let student = Student::new(input.name.clone(), input.total_marks, input.num_subjects);
    
//     STUDENTS.with(|students| {
//         students.borrow_mut().insert(input.name.clone(), student.clone());
//     });
    
//     STUDENT_COUNTER.with(|counter| {
//         *counter.borrow_mut() += 1;
//     });

//     Ok(student)
// }

// #[query]
// pub fn get_student(name: String) -> Option<Student> {
//     STUDENTS.with(|students| {
//         students.borrow().get(&name).cloned()
//     })
// }

// #[query]
// pub fn get_all_students() -> Vec<Student> {
//     STUDENTS.with(|students| {
//         students.borrow().values().cloned().collect()
//     })
// }

// #[query]
// pub fn get_report_card(name: String) -> Option<ReportCard> {
//     STUDENTS.with(|students| {
//         students.borrow().get(&name).map(|student| {
//             ReportCard {
//                 student: student.clone(),
//                 report_html: student.to_report_html(),
//             }
//         })
//     })
// }

// #[query]
// pub fn get_student_count() -> u64 {
//     STUDENT_COUNTER.with(|counter| *counter.borrow())
// }

// #[update]
// pub fn delete_student(name: String) -> Result<String, String> {
//     STUDENTS.with(|students| {
//         match students.borrow_mut().remove(&name) {
//             Some(_) => {
//                 STUDENT_COUNTER.with(|counter| {
//                     *counter.borrow_mut() -= 1;
//                 });
//                 Ok(format!("Student '{}' deleted successfully", name))
//             }
//             None => Err(format!("Student '{}' not found", name))
//         }
//     })
// }

// #[query]
// pub fn get_grade_statistics() -> HashMap<String, u32> {
//     STUDENTS.with(|students| {
//         let mut stats = HashMap::new();
//         stats.insert("A".to_string(), 0);
//         stats.insert("B".to_string(), 0);
//         stats.insert("C".to_string(), 0);
//         stats.insert("D".to_string(), 0);
        
//         for student in students.borrow().values() {
//             *stats.entry(student.grade.clone()).or_insert(0) += 1;
//         }
        
//         stats
//     })
// }

// // Export the candid interface
// ic_cdk::export_candid!();


use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Student {
    pub name: String,
    pub total_marks: f64,
    pub num_subjects: u32,
    pub average: f64,
    pub grade: String,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FormattedStudent {
    pub name: String,
    pub total_marks: f64,
    pub num_subjects: u32,
    pub average: f64,
    pub grade: String,
    pub performance_message: String,
    pub timestamp: String,
}

impl From<Student> for FormattedStudent {
    fn from(student: Student) -> Self {
        let performance_message = match student.grade.as_str() {
            "A" => "🌟 Outstanding Performance! Excellent work!".to_string(),
            "B" => "👏 Great Job! Keep up the good work!".to_string(),
            "C" => "📈 Good effort! There's room for improvement.".to_string(),
            _ => "📚 Needs more focus and study time.".to_string(),
        };

        FormattedStudent {
            name: student.name,
            total_marks: student.total_marks,
            num_subjects: student.num_subjects,
            average: student.average,
            grade: format!("Grade {}", student.grade),
            performance_message,
            timestamp: format!("Generated on: {}", student.timestamp),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct StudentInput {
    pub name: String,
    pub total_marks: f64,
    pub num_subjects: u32,
}

#[derive(CandidType, Deserialize)]
pub struct ReportCard {
    pub student: Student,
    pub report_html: String,
}

thread_local! {
    static STUDENTS: RefCell<HashMap<String, Student>> = RefCell::new(HashMap::new());
    static STUDENT_COUNTER: RefCell<u64> = RefCell::new(0);
}

impl Student {
    fn new(name: String, total_marks: f64, num_subjects: u32) -> Self {
        let average = calculate_average(total_marks, num_subjects);
        let grade = assign_grade(average);
        
        Student {
            name,
            total_marks,
            num_subjects,
            average,
            grade,
            timestamp: time(),
        }
    }

    fn to_report_html(&self) -> String {
        let grade_color = match self.grade.as_str() {
            "A" => "#00ff88",
            "B" => "#00ccff", 
            "C" => "#ffaa00",
            _ => "#ff4444"
        };
        
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Official Report Card - {}</title>
                <style>
                    @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;600;700&display=swap');
                    
                    body {{ 
                        font-family: 'Poppins', sans-serif; 
                        margin: 0; 
                        padding: 40px;
                        background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
                        min-height: 100vh;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    }}
                    
                    .report-card {{ 
                        background: white;
                        border-radius: 20px;
                        padding: 40px;
                        max-width: 700px;
                        width: 100%;
                        box-shadow: 0 25px 50px rgba(0,0,0,0.25);
                        position: relative;
                        overflow: hidden;
                    }}
                    
                    .report-card::before {{
                        content: '';
                        position: absolute;
                        top: 0;
                        left: 0;
                        right: 0;
                        height: 8px;
                        background: linear-gradient(90deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4);
                    }}
                    
                    .school-header {{
                        text-align: center;
                        margin-bottom: 30px;
                        color: #2c3e50;
                    }}
                    
                    .school-name {{
                        font-size: 24px;
                        font-weight: 700;
                        margin-bottom: 5px;
                        text-transform: uppercase;
                        letter-spacing: 2px;
                    }}
                    
                    .report-title {{
                        font-size: 20px;
                        color: #7f8c8d;
                        font-weight: 300;
                        margin-bottom: 30px;
                    }}
                    
                    .student-info {{
                        display: grid;
                        grid-template-columns: 1fr 1fr;
                        gap: 20px;
                        margin-bottom: 30px;
                    }}
                    
                    .info-item {{
                        background: #f8f9fa;
                        padding: 20px;
                        border-radius: 12px;
                        border-left: 4px solid #3498db;
                    }}
                    
                    .info-label {{
                        font-size: 14px;
                        color: #7f8c8d;
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                        margin-bottom: 8px;
                    }}
                    
                    .info-value {{
                        font-size: 18px;
                        color: #2c3e50;
                        font-weight: 600;
                    }}
                    
                    .grade-section {{
                        text-align: center;
                        margin: 40px 0;
                        padding: 30px;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        border-radius: 15px;
                        color: white;
                    }}
                    
                    .grade-label {{
                        font-size: 16px;
                        font-weight: 300;
                        margin-bottom: 10px;
                        opacity: 0.9;
                    }}
                    
                    .grade-value {{
                        font-size: 72px;
                        font-weight: 700;
                        color: {};
                        text-shadow: 0 4px 8px rgba(0,0,0,0.3);
                        margin: 0;
                    }}
                    
                    .grade-description {{
                        font-size: 18px;
                        margin-top: 10px;
                        font-weight: 400;
                    }}
                    
                    .performance-bar {{
                        width: 100%;
                        height: 10px;
                        background: rgba(255,255,255,0.3);
                        border-radius: 10px;
                        margin-top: 20px;
                        overflow: hidden;
                    }}
                    
                    .performance-fill {{
                        height: 100%;
                        background: {};
                        width: {}%;
                        border-radius: 10px;
                        transition: width 2s ease;
                    }}
                    
                    .grade-scale {{
                        background: #ecf0f1;
                        padding: 25px;
                        border-radius: 12px;
                        margin-top: 30px;
                    }}
                    
                    .scale-title {{
                        font-size: 16px;
                        font-weight: 600;
                        color: #2c3e50;
                        margin-bottom: 15px;
                        text-align: center;
                    }}
                    
                    .scale-grid {{
                        display: grid;
                        grid-template-columns: repeat(4, 1fr);
                        gap: 15px;
                    }}
                    
                    .scale-item {{
                        text-align: center;
                        padding: 15px;
                        background: white;
                        border-radius: 8px;
                        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                    }}
                    
                    .scale-grade {{
                        font-size: 24px;
                        font-weight: 700;
                        margin-bottom: 5px;
                    }}
                    
                    .scale-range {{
                        font-size: 12px;
                        color: #7f8c8d;
                        font-weight: 500;
                    }}
                    
                    .footer {{
                        text-align: center;
                        margin-top: 40px;
                        padding-top: 20px;
                        border-top: 2px solid #ecf0f1;
                        color: #7f8c8d;
                        font-size: 14px;
                    }}
                    
                    .signature-line {{
                        margin-top: 30px;
                        text-align: right;
                    }}
                    
                    .signature {{
                        border-top: 2px solid #bdc3c7;
                        display: inline-block;
                        padding-top: 5px;
                        margin-top: 20px;
                        min-width: 200px;
                        font-size: 14px;
                        color: #7f8c8d;
                    }}
                </style>
            </head>
            <body>
                <div class="report-card">
                    <div class="school-header">
                        <div class="school-name">🎓 Academic Excellence Institute</div>
                        <div class="report-title">Official Student Report Card</div>
                    </div>
                    
                    <div class="student-info">
                        <div class="info-item">
                            <div class="info-label">Student Name</div>
                            <div class="info-value">{}</div>
                        </div>
                        <div class="info-item">
                            <div class="info-label">Total Marks</div>
                            <div class="info-value">{}</div>
                        </div>
                        <div class="info-item">
                            <div class="info-label">Number of Subjects</div>
                            <div class="info-value">{}</div>
                        </div>
                        <div class="info-item">
                            <div class="info-label">Average Score</div>
                            <div class="info-value">{:.1}%</div>
                        </div>
                    </div>
                    
                    <div class="grade-section">
                        <div class="grade-label">Final Grade</div>
                        <div class="grade-value">{}</div>
                        <div class="grade-description">{}</div>
                        <div class="performance-bar">
                            <div class="performance-fill"></div>
                        </div>
                    </div>
                    
                    <div class="grade-scale">
                        <div class="scale-title">📊 Grading Scale</div>
                        <div class="scale-grid">
                            <div class="scale-item">
                                <div class="scale-grade" style="color: #00ff88;">A</div>
                                <div class="scale-range">90-100<br>Excellent</div>
                            </div>
                            <div class="scale-item">
                                <div class="scale-grade" style="color: #00ccff;">B</div>
                                <div class="scale-range">75-89<br>Good</div>
                            </div>
                            <div class="scale-item">
                                <div class="scale-grade" style="color: #ffaa00;">C</div>
                                <div class="scale-range">60-74<br>Average</div>
                            </div>
                            <div class="scale-item">
                                <div class="scale-grade" style="color: #ff4444;">D</div>
                                <div class="scale-range">Below 60<br>Needs Work</div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="footer">
                        <p>This report card is generated automatically and reflects the student's current academic performance.</p>
                        <div class="signature-line">
                            <div class="signature">Academic Administrator</div>
                        </div>
                    </div>
                </div>
            </body>
            </html>
            "#,
            self.name,
            grade_color,
            grade_color, 
            self.average.min(100.0),
            self.name, 
            self.total_marks, 
            self.num_subjects, 
            self.average,
            self.grade,
            match self.grade.as_str() {
                "A" => "Outstanding Performance! 🌟",
                "B" => "Great Job! Keep it up! 👏",
                "C" => "Good effort! Room for improvement 📈",
                _ => "Needs more focus and study time 📚"
            }
        )
    }
}

// Custom function to calculate average
fn calculate_average(total_marks: f64, num_subjects: u32) -> f64 {
    if num_subjects == 0 {
        return 0.0;
    }
    total_marks / num_subjects as f64
}

// Function to assign grade based on average
fn assign_grade(average: f64) -> String {
    match average {
        avg if avg >= 90.0 => "A".to_string(),
        avg if avg >= 75.0 => "B".to_string(),
        avg if avg >= 60.0 => "C".to_string(),
        _ => "D".to_string(),
    }
}

#[update]
pub fn add_student(input: StudentInput) -> Result<Student, String> {
    if input.name.trim().is_empty() {
        return Err("❌ Student name cannot be empty".to_string());
    }
    
    if input.total_marks < 0.0 {
        return Err("❌ Total marks cannot be negative".to_string());
    }
    
    if input.num_subjects == 0 {
        return Err("❌ Number of subjects must be greater than 0".to_string());
    }

    let student = Student::new(input.name.clone(), input.total_marks, input.num_subjects);
    
    STUDENTS.with(|students| {
        students.borrow_mut().insert(input.name.clone(), student.clone());
    });
    
    STUDENT_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
    });

    Ok(student)
}

#[query]
pub fn get_student_formatted(name: String) -> Option<FormattedStudent> {
    STUDENTS.with(|students| {
        students.borrow().get(&name).map(|student| FormattedStudent::from(student.clone()))
    })
}

#[query]
pub fn get_all_students_formatted() -> Vec<FormattedStudent> {
    STUDENTS.with(|students| {
        students.borrow().values().map(|student| FormattedStudent::from(student.clone())).collect()
    })
}

#[query]
pub fn get_report_card(name: String) -> Option<ReportCard> {
    STUDENTS.with(|students| {
        students.borrow().get(&name).map(|student| {
            ReportCard {
                student: student.clone(),
                report_html: student.to_report_html(),
            }
        })
    })
}

#[query]
pub fn get_student_count() -> u64 {
    STUDENT_COUNTER.with(|counter| *counter.borrow())
}

#[update]
pub fn delete_student(name: String) -> Result<String, String> {
    STUDENTS.with(|students| {
        match students.borrow_mut().remove(&name) {
            Some(_) => {
                STUDENT_COUNTER.with(|counter| {
                    *counter.borrow_mut() -= 1;
                });
                Ok(format!("✅ Student '{}' deleted successfully", name))
            }
            None => Err(format!("❌ Student '{}' not found", name))
        }
    })
}

#[query]
pub fn get_grade_statistics() -> HashMap<String, u32> {
    STUDENTS.with(|students| {
        let mut stats = HashMap::new();
        stats.insert("A".to_string(), 0);
        stats.insert("B".to_string(), 0);
        stats.insert("C".to_string(), 0);
        stats.insert("D".to_string(), 0);
        
        for student in students.borrow().values() {
            *stats.entry(student.grade.clone()).or_insert(0) += 1;
        }
        
        stats
    })
}

// Export the candid interface
ic_cdk::export_candid!();