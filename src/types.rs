pub fn name<T>(_: &T) -> &str {
	std::any::type_name::<T>()
}

pub fn basename<T>(object: &T) -> &str {
	let obj_name = name(object);
	obj_name.split("::").last().unwrap_or(obj_name)
}
