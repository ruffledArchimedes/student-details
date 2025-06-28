export const idlFactory = ({ IDL }) => {
  const StudentInput = IDL.Record({
    'name' : IDL.Text,
    'num_subjects' : IDL.Nat32,
    'total_marks' : IDL.Float64,
  });
  const Student = IDL.Record({
    'name' : IDL.Text,
    'average' : IDL.Float64,
    'num_subjects' : IDL.Nat32,
    'total_marks' : IDL.Float64,
    'grade' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : Student, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const ReportCard = IDL.Record({
    'report_html' : IDL.Text,
    'student' : Student,
  });
  return IDL.Service({
    'add_student' : IDL.Func([StudentInput], [Result], []),
    'delete_student' : IDL.Func([IDL.Text], [Result_1], []),
    'get_all_students' : IDL.Func([], [IDL.Vec(Student)], ['query']),
    'get_grade_statistics' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32))],
        ['query'],
      ),
    'get_report_card' : IDL.Func([IDL.Text], [IDL.Opt(ReportCard)], ['query']),
    'get_student' : IDL.Func([IDL.Text], [IDL.Opt(Student)], ['query']),
    'get_student_count' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
