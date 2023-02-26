
src = print_string_list.c
lib = libprintlist.so

$(lib):$(src)
	gcc -shared $< -o $@

test:$(lib)
	otool -L $<
	nm $<

clean:
	rm $(lib)
