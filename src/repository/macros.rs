macro_rules! find_all {
    ($t:expr, $c:expr) => {
        $t.load($c)
    };
}

macro_rules! get_one {
    ($t:expr, $c:expr, $id:expr) => {
        $t.find($id).first($c)
    };
}

macro_rules! create {
    ($t:expr, $c:expr, $val:expr) => {
        diesel::insert_into($t).values($val).get_result($c)
    };
}

macro_rules! update {
    ($t:expr, $c:expr, $id:expr, $val:expr) => {
        diesel::update($t.find($id)).set($val).get_result($c)
    };
}

macro_rules! delete {
    ($t:expr, $c:expr, $id:expr) => {
        diesel::delete($t.find($id)).execute($c)
    };
}