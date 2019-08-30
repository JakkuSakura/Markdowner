# Easy_dom
Easy_dom provides you with a easy approach to build up your own node tree automatically, which means you only need to 
write your rule file like I pasted below.

```text
#note text is the root of a passage. it contains a list of blocks
#def text
#mem data ARRAY
#grammar block{0,-1,push_into(data)}
#end_def

#def block
#grammar char{1,1,replace_this()}
#end_def

```

## Notes
Notes begin with `#` followed by note tag name. 

### Comment
Anything after `#note` in the same line will be omitted. It's designed for comments.

### Define a new node with its name
Definition process begins with `#def name`. Nodes' names are necessary all the time, as they are used for identification.

### Specify a node's content
You may need several `#mem name type` to assign a node's content, which means that you can use its content, or members, later.

Here are what you may use:
 - STRING
 - INTEGER
 - FLOAT
 - POINTER
 - ARRAY
 - NONE

### Define your grammar

A very essential part of a rule is its grammars. 

You have to use `#grammar` to start a grammar definition, which contains chucks like this `block{0,-1,push_into(data)}`.
Chucks are not allowed to contain whitespaces for the sake of programming simplicity.

For the same reason, a chuck has to have a name, a match least number, a match most number and an action.

Name could be a self-defined node name or a built-in name.

Match least numbers and match most numbers are integers. The former has to be less than the latter, 
except when match most number is a negative number, meaning infinite. Easy_dom will try to match every chuck as you set.
If one chuck's repeat times is between match least number and most number, this chuck will be accepted. As soon as all 
the chucks are accepted, THIS grammar will be successful and thus THIS node will be build. 

Actions are usually single-parametered or non-parametered, at this very early stage. 
While one chuck is accepted, its action will be executed.

These are built-in actions

`push_into(name)`: push THIS node into `name`, which is an array specified above.

`nothing()`: literally nothing to do

`replace_this()`: replace parent node with this node