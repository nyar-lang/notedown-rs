use notedown_ast::ASTNode;

pub fn regroup_list_view(lists: &[(usize, &str, Vec<ASTNode>)]) -> Vec<ASTNode> {
    println!("{:#?}", lists);
    vec![]
}

pub fn regroup_table_view(table: &[Vec<Vec<ASTNode>>]) -> Vec<ASTNode> {
    for line in table {
        for item in line {
            println!("{:?}", item)
        }
    }
    vec![]
}