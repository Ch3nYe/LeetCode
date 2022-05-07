use std::borrow::Borrow;

pub struct Solution;
// 其实也可以称为回溯法，只是没有任何可以用于剪枝的约束
impl Solution {
    pub fn dfs(graph: &Vec<Vec<i32>>, // The original graph
               curr: i32, n: i32, // current node idx
               stack: &mut Vec<i32>, // stack store path
               ans: &mut Vec<Vec<i32>> // ans store all feasible path
    ) {
        if curr==n {
            ans.push(stack.clone());
            return;
        }
        for &next in graph[curr as usize].iter() {
            stack.push(next);
            Self::dfs(graph, next, n, stack, ans);
            stack.pop();
        }

    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut stack = Vec::new();
        let mut ans:Vec<Vec<i32>> = vec![];
        stack.push(0);

        Self::dfs(&graph, 0, (graph.len() - 1) as i32, &mut stack, &mut ans);

        return ans;
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::all_paths_source_target(graph)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}