import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ReportCard { 'report_html' : string, 'student' : Student }
export type Result = { 'Ok' : Student } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export interface Student {
  'name' : string,
  'average' : number,
  'num_subjects' : number,
  'total_marks' : number,
  'grade' : string,
  'timestamp' : bigint,
}
export interface StudentInput {
  'name' : string,
  'num_subjects' : number,
  'total_marks' : number,
}
export interface _SERVICE {
  'add_student' : ActorMethod<[StudentInput], Result>,
  'delete_student' : ActorMethod<[string], Result_1>,
  'get_all_students' : ActorMethod<[], Array<Student>>,
  'get_grade_statistics' : ActorMethod<[], Array<[string, number]>>,
  'get_report_card' : ActorMethod<[string], [] | [ReportCard]>,
  'get_student' : ActorMethod<[string], [] | [Student]>,
  'get_student_count' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
