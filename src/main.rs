mod symbols;
mod segments;

use std::env;
use std::process::Command;

use segments::connection::connect_last;
use segments::connection::connect_segments;

use segments::error::error_segment;
use segments::path::path_segments;
use segments::virtualenv::virtualenv_segment;

use segments::colors;

fn main() {
    let args: Vec<_> = env::args().collect();
    let error: bool = (args.len() > 1) && (args[1] != "0") ;

    let clean_colors : String = "\\[\x1b[0m\\]".to_string();
    let first_segment : String = error_segment(error);
    let virtualenv_segment : Option<String> = virtualenv_segment();
    let second_segment : String = path_segments();
    let context = Command::new("kubectl")
        .arg("config")
        .arg("current-context")
        .output()
        .expect("failed to execute process").stdout;

    let cluster = String::from_utf8_lossy(&context);
    let cluster = cluster.trim();

    let cluster_bg = if cluster == "minikube" {
        "2"
    }
    else {
        "1"
    };

    print!("{clean}{error} ",
             clean=clean_colors,
             error=first_segment,
             );

    match virtualenv_segment {
        Some(x) => {
            print!("{c1}{virtualenv} {c2}",
                   virtualenv=x,
                   c1=connect_segments(colors::LAST_RESULT_BG,
                                       colors::VIRTUALENV_BG),
                   c2=connect_segments(colors::VIRTUALENV_BG,
                                       cluster_bg),
                   )
        }
        None => {
            print!("{c}",
                     c=connect_segments(colors::LAST_RESULT_BG,
                                        cluster_bg),
                     )
        }
    }


    print!("{color_base}{cluster_color}{color_end} {cluster} {connector}",
           color_base="\\[\x1b[1;37m\x1b[1;4",
           cluster_color=cluster_bg,
           color_end="m\\]",
           cluster=cluster,
           connector=connect_segments(cluster_bg, colors::PATH_BG));

    print!("{path} {c}{clean} ",
             c=connect_last(colors::PATH_BG),
             clean=clean_colors,
             path=second_segment);
}
