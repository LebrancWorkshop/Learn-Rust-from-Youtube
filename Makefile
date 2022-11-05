# n: new rust project. 
n:
	sh new.sh $(r)

# r: run rust project. 
r:
	sh main.sh $(r)

# c: check rust project. 
c:
	sh check.sh $(r)

.PHONY: n r c